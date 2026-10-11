use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager, WebviewWindow};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

pub const EVENT_NAME: &str = "podman-event";
pub const STATUS_NAME: &str = "podman-events-status";

const REQUEST: &str = concat!(
    "GET /events HTTP/1.1\r\n",
    "Host: localhost\r\n",
    "Accept: application/json\r\n",
    "Connection: keep-alive\r\n",
    "\r\n"
);

const INITIAL_BACKOFF: Duration = Duration::from_secs(1);
const MAX_BACKOFF: Duration = Duration::from_secs(15);

#[derive(Clone, Debug, Serialize)]
pub struct PodmanEvent {
    pub kind: String,
    pub action: String,
    pub id: String,
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct EventsStatus {
    pub connected: bool,
    pub socket: Option<String>,
    pub detail: Option<String>,
}

/// Penyimpan status langganan event milik aplikasi. Frontend membacanya lewat
/// perintah `events_status` supaya tidak kehilangan pembaruan yang terkirim
/// sebelum listener di webview sempat terpasang.
#[derive(Default)]
pub struct StatusStore(pub std::sync::Mutex<EventsStatus>);

pub fn publish(window: &WebviewWindow, value: EventsStatus) {
    if let Some(store) = window.try_state::<StatusStore>() {
        if let Ok(mut current) = store.0.lock() {
            *current = value.clone();
        }
    }

    let _ = window.emit(STATUS_NAME, value);
}

pub fn current(window: &WebviewWindow) -> EventsStatus {
    window
        .try_state::<StatusStore>()
        .and_then(|store| store.0.lock().ok().map(|current| current.clone()))
        .unwrap_or_default()
}

#[derive(Debug, Deserialize)]
struct RawEvent {
    #[serde(rename = "Type")]
    kind: Option<String>,
    #[serde(rename = "Action")]
    action: Option<String>,
    status: Option<String>,
    id: Option<String>,
    #[serde(rename = "Actor")]
    actor: Option<RawActor>,
    time: Option<i64>,
    time_nano: Option<i64>,
}

#[derive(Debug, Default, Deserialize)]
struct RawActor {
    #[serde(rename = "ID")]
    id: Option<String>,
    #[serde(rename = "Attributes")]
    attributes: Option<HashMap<String, String>>,
}

/// Jalankan penulis langganan event di belakang layar, otomatis menyambung
/// ulang dengan backoff bila socket mati atau podman di-restart.
pub fn spawn(window: WebviewWindow) {
    tauri::async_runtime::spawn(async move {
        let mut backoff = INITIAL_BACKOFF;

        loop {
            let candidates = socket_candidates();
            let mut connected = false;
            let mut last_error = "tidak ada socket Podman/Docker yang ditemukan".to_string();

            for socket in &candidates {
                let status_window = window.clone();
                let event_window = window.clone();

                let result = run_stream(
                    socket,
                    || {
                        connected = true;
                        println!("event stream tersambung ke {socket}");
                        publish(
                            &status_window,
                            EventsStatus {
                                connected: true,
                                socket: Some(socket.to_string()),
                                detail: None,
                            },
                        );
                    },
                    |event| {
                        let _ = event_window.emit(EVENT_NAME, event);
                    },
                )
                .await;

                if let Err(err) = result {
                    last_error = err;
                }
            }

            publish(
                &window,
                EventsStatus {
                    connected: false,
                    socket: None,
                    detail: Some(last_error.clone()),
                },
            );
            eprintln!("event stream terputus: {last_error}");

            backoff = if connected {
                INITIAL_BACKOFF
            } else {
                (backoff * 2).min(MAX_BACKOFF)
            };

            tokio::time::sleep(backoff).await;
        }
    });
}

/// Baca `/events` mentah dari socket unix: `GET` -> header -> body chunked
/// -> NDJSON. Setiap baris JSON yang tidak terbaca hanya dilewati, tidak
/// mematikan stream (penting untuk tipe khas Podman seperti `pod`).
pub async fn run_stream<C, F>(
    socket: &str,
    on_connected: C,
    mut on_event: F,
) -> Result<(), String>
where
    C: FnOnce(),
    F: FnMut(PodmanEvent),
{
    let mut stream = UnixStream::connect(socket)
        .await
        .map_err(|err| format!("tidak bisa membuka {socket}: {err}"))?;

    stream
        .write_all(REQUEST.as_bytes())
        .await
        .map_err(|err| format!("tidak bisa mengirim permintaan ke {socket}: {err}"))?;

    let mut network = Vec::new();
    let mut body = Vec::new();
    let mut decoder;

    loop {
        if let Some(end) = find_header_end(&network) {
            let head = String::from_utf8_lossy(&network[..end]).into_owned();
            network.drain(..end);

            decoder = match status_code(&head) {
                Some(200) => BodyDecoder::from_headers(&head),
                other => {
                    return Err(format!(
                        "{socket} membalas {}",
                        other
                            .map(|code| code.to_string())
                            .unwrap_or_else(|| "status tak dikenal".to_string())
                    ));
                }
            };

            on_connected();
            break;
        }

        read_more(&mut stream, &mut network).await?;
    }

    loop {
        read_more(&mut stream, &mut network).await?;

        if !decoder.feed(&mut network, &mut body)? {
            return Err(format!("{socket} menutup stream event"));
        }

        drain_lines(&mut body, &mut on_event);
    }
}

fn socket_candidates() -> Vec<String> {
    let mut candidates: Vec<String> = Vec::new();

    if let Ok(host) = std::env::var("DOCKER_HOST") {
        if let Some(path) = host.strip_prefix("unix://") {
            if !path.is_empty() {
                candidates.push(path.to_string());
            }
        }
    }

    if let Some(runtime_dir) = std::env::var_os("XDG_RUNTIME_DIR") {
        let runtime_dir = PathBuf::from(runtime_dir);
        push_path(&mut candidates, runtime_dir.join("podman/podman.sock"));
        push_path(&mut candidates, runtime_dir.join("docker.sock"));
    }

    push_path(&mut candidates, PathBuf::from("/run/podman/podman.sock"));
    push_path(&mut candidates, PathBuf::from("/var/run/docker.sock"));

    candidates
}

fn push_path(candidates: &mut Vec<String>, path: PathBuf) {
    let value = path.to_string_lossy().into_owned();
    if !candidates.contains(&value) {
        candidates.push(value);
    }
}

async fn read_more(stream: &mut UnixStream, buffer: &mut Vec<u8>) -> Result<(), String> {
    let mut chunk = [0u8; 8192];
    let read = stream
        .read(&mut chunk)
        .await
        .map_err(|err| format!("gagal membaca stream: {err}"))?;

    if read == 0 {
        return Err("koneksi ditutup oleh podman".to_string());
    }

    buffer.extend_from_slice(&chunk[..read]);
    Ok(())
}

fn find_header_end(buffer: &[u8]) -> Option<usize> {
    buffer
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|pos| pos + 4)
}

fn find_crlf(buffer: &[u8]) -> Option<usize> {
    buffer.windows(2).position(|window| window == b"\r\n")
}

fn status_code(head: &str) -> Option<u16> {
    head.lines()
        .next()?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()
}

enum BodyDecoder {
    Raw,
    Chunked { state: ChunkState },
}

enum ChunkState {
    Size,
    Data(usize),
    CrLf,
}

impl BodyDecoder {
    fn from_headers(head: &str) -> Self {
        let chunked = head.lines().any(|line| {
            let lower = line.to_ascii_lowercase();
            lower.starts_with("transfer-encoding:") && lower.contains("chunked")
        });

        if chunked {
            BodyDecoder::Chunked {
                state: ChunkState::Size,
            }
        } else {
            BodyDecoder::Raw
        }
    }

    /// Salin byte jaringan yang sudah utuh ke `body`. `Ok(false)` berarti
    /// badan resmi berakhir (chunk terakhir berukuran nol).
    fn feed(&mut self, network: &mut Vec<u8>, body: &mut Vec<u8>) -> Result<bool, String> {
        match self {
            BodyDecoder::Raw => {
                body.append(network);
                Ok(true)
            }
            BodyDecoder::Chunked { state } => loop {
                match state {
                    ChunkState::Size => {
                        let Some(pos) = find_crlf(network) else {
                            return Ok(true);
                        };

                        let raw = String::from_utf8_lossy(&network[..pos]);
                        let hex = raw.split(';').next().unwrap_or("").trim();
                        let size = usize::from_str_radix(hex, 16)
                            .map_err(|_| format!("ukuran chunk tidak valid: {hex:?}"))?;

                        network.drain(..pos + 2);

                        if size == 0 {
                            return Ok(false);
                        }

                        *state = ChunkState::Data(size);
                    }
                    ChunkState::Data(size) => {
                        if network.len() < *size {
                            return Ok(true);
                        }

                        let size = *size;
                        body.extend_from_slice(&network[..size]);
                        network.drain(..size);
                        *state = ChunkState::CrLf;
                    }
                    ChunkState::CrLf => {
                        if network.len() < 2 {
                            return Ok(true);
                        }

                        network.drain(..2);
                        *state = ChunkState::Size;
                    }
                }
            },
        }
    }
}

fn drain_lines<F>(body: &mut Vec<u8>, on_event: &mut F)
where
    F: FnMut(PodmanEvent),
{
    while let Some(pos) = body.iter().position(|byte| *byte == b'\n') {
        let mut line: Vec<u8> = body.drain(..=pos).collect();

        if line.last() == Some(&b'\n') {
            line.pop();
        }
        if line.last() == Some(&b'\r') {
            line.pop();
        }

        if line.is_empty() {
            continue;
        }

        match parse_event(&line) {
            Ok(event) => on_event(event),
            Err(err) => eprintln!("event dilewati, JSON tidak terbaca: {err}"),
        }
    }
}

fn parse_event(line: &[u8]) -> Result<PodmanEvent, String> {
    let raw: RawEvent =
        serde_json::from_slice(line).map_err(|err| format!("{err} pada {}", line.len()))?;

    let actor = raw.actor.unwrap_or_default();
    let attributes = actor.attributes.unwrap_or_default();

    let name = attributes
        .get("name")
        .cloned()
        .unwrap_or_default();
    let id = actor
        .id
        .filter(|value| !value.is_empty())
        .or(raw.id)
        .unwrap_or_default();
    let kind = raw.kind.unwrap_or_else(|| "unknown".to_string());
    let action = raw
        .action
        .or(raw.status)
        .unwrap_or_else(|| "unknown".to_string());
    let timestamp = raw
        .time_nano
        .map(|value| (value / 1_000_000) as u64)
        .or_else(|| raw.time.map(|value| (value * 1000) as u64))
        .unwrap_or_default();

    Ok(PodmanEvent {
        kind,
        action,
        id,
        name,
        attributes,
        timestamp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    fn best_socket() -> Option<String> {
        socket_candidates()
            .into_iter()
            .find(|path| PathBuf::from(path).exists())
    }

    #[tokio::test]
    async fn reads_container_and_pod_events() {
        let Some(socket) = best_socket() else {
            eprintln!("lewat: tidak ada socket podman/docker");
            return;
        };

        if UnixStream::connect(&socket).await.is_err() {
            eprintln!("lewat: socket {socket} tidak bisa dibuka");
            return;
        }

        let collected: Arc<Mutex<Vec<PodmanEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let sink = collected.clone();
        let reader = tokio::spawn({
            let socket = socket.clone();
            async move {
                let _ = run_stream(
                    &socket,
                    || eprintln!("tersambung ke {socket}"),
                    move |event| sink.lock().unwrap().push(event),
                )
                .await;
            }
        });

        tokio::time::sleep(Duration::from_millis(300)).await;

        let url = format!("unix://{socket}");
        let suffix = std::process::id();
        let volume = format!("evt-volume-{suffix}");
        let pod = format!("evt-pod-{suffix}");

        for args in [
            vec!["volume", "create", &volume],
            vec!["pod", "create", "--name", &pod],
            vec!["pod", "rm", "-f", &pod],
            vec!["volume", "rm", &volume],
        ] {
            let _ = tokio::process::Command::new("podman")
                .arg("--url")
                .arg(&url)
                .args(&args)
                .output()
                .await;
        }

        let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
        let wanted = ["volume", "pod"];
        loop {
            let (done, received) = {
                let events = collected.lock().unwrap();
                let done = wanted.iter().all(|kind| {
                    events
                        .iter()
                        .any(|event| event.kind == *kind && event.name.starts_with("evt-"))
                });
                let received = events
                    .iter()
                    .map(|event| {
                        (
                            event.kind.clone(),
                            event.action.clone(),
                            event.name.clone(),
                        )
                    })
                    .collect::<Vec<_>>();

                (done, received)
            };

            if done {
                reader.abort();
                return;
            }

            if tokio::time::Instant::now() >= deadline {
                reader.abort();
                panic!("event tidak lengkap, diterima: {received:?}");
            }

            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }
}
