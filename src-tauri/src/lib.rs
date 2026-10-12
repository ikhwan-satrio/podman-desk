mod utils;

use tauri::Manager;
use tokio::process::Command;

use bollard::{
    container::LogOutput,
    models::{ContainerCreateBody, HostConfig, PortBinding},
    plugin::{ContainerSummary, ImageSummary, Network, Volume},
    query_parameters::{
        CreateContainerOptions, ListContainersOptionsBuilder, ListImagesOptionsBuilder,
        ListNetworksOptionsBuilder, ListVolumesOptionsBuilder, LogsOptionsBuilder,
        RemoveContainerOptions, RemoveImageOptions, RemoveVolumeOptions, RenameContainerOptions,
        RestartContainerOptions, StopContainerOptions,
    },
};
use futures_util::StreamExt;
use serde_json::Value;
use utils::docker::connect_docker;
use utils::events;

#[tauri::command]
async fn get_containers() -> Result<Vec<ContainerSummary>, String> {
    let dock = connect_docker().await?;

    let opt = ListContainersOptionsBuilder::default().all(true).build();

    let containers = dock
        .list_containers(Some(opt))
        .await
        .map_err(|e| e.to_string())?;

    Ok(containers)
}

#[tauri::command]
async fn get_container(id: String) -> Result<Value, String> {
    let dock = connect_docker().await?;

    let container = dock
        .inspect_container(&id, None)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_value(container).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_container_logs(id: String) -> Result<String, String> {
    let dock = connect_docker().await?;

    let options = LogsOptionsBuilder::default()
        .stdout(true)
        .stderr(true)
        .tail("500")
        .timestamps(true)
        .build();

    let mut stream = dock.logs(&id, Some(options));
    let mut output = String::new();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(LogOutput::StdOut { message }) | Ok(LogOutput::StdErr { message }) => {
                output.push_str(&String::from_utf8_lossy(&message));
            }
            Ok(_) => {}
            Err(err) => return Err(err.to_string()),
        }
    }

    Ok(output)
}

#[tauri::command]
async fn get_images() -> Result<Vec<ImageSummary>, String> {
    let dock = connect_docker().await?;
    let opt = ListImagesOptionsBuilder::default().all(true).build();

    let images = dock
        .list_images(Some(opt))
        .await
        .map_err(|e| e.to_string())?;

    Ok(images)
}

#[tauri::command]
async fn get_image(id: String) -> Result<Value, String> {
    let dock = connect_docker().await?;

    let image = dock
        .inspect_image(&id)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_value(image).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_image_history(id: String) -> Result<Value, String> {
    let dock = connect_docker().await?;

    let history = dock
        .image_history(&id)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::to_value(history).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_image_filesystem(id: String) -> Result<utils::image_fs::ImageFs, String> {
    let dock = connect_docker().await?;
    utils::image_fs::read_image_fs(&dock, &id).await
}

#[tauri::command]
async fn get_volumes() -> Result<Option<Vec<Volume>>, String> {
    let dock = connect_docker().await?;
    let opt = ListVolumesOptionsBuilder::default().build();

    let volumes = dock
        .list_volumes(Some(opt))
        .await
        .map_err(|e| e.to_string())?;

    Ok(volumes.volumes)
}

#[tauri::command]
async fn get_networks() -> Result<Vec<Network>, String> {
    let dock = connect_docker().await?;
    let opt = ListNetworksOptionsBuilder::default().build();

    let networks = dock
        .list_networks(Some(opt))
        .await
        .map_err(|e| e.to_string())?;

    Ok(networks)
}

#[tauri::command]
fn events_status(window: tauri::WebviewWindow) -> events::EventsStatus {
    events::current(&window)
}

#[tauri::command]
async fn get_pods() -> Result<Value, String> {
    let output = Command::new("podman")
        .args(["pod", "ps", "--format", "json"])
        .output()
        .await
        .map_err(|e| format!("Gagal mengeksekusi podman: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let pods: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Gagal parse JSON podman: {}", e))?;

    Ok(pods)
}

#[tauri::command]
async fn start_container(id: String) -> Result<(), String> {
    let dock = connect_docker().await?;
    dock.start_container(&id, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn stop_container(id: String) -> Result<(), String> {
    let dock = connect_docker().await?;
    let opt = StopContainerOptions { t: Some(10), ..Default::default() };
    dock.stop_container(&id, Some(opt)).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn restart_container(id: String) -> Result<(), String> {
    let dock = connect_docker().await?;
    let opt = RestartContainerOptions { t: Some(10), ..Default::default() };
    dock.restart_container(&id, Some(opt)).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_container(id: String, force: Option<bool>) -> Result<(), String> {
    let dock = connect_docker().await?;
    let opt = RemoveContainerOptions {
        force: force.unwrap_or(false),
        ..Default::default()
    };
    dock.remove_container(&id, Some(opt)).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn rename_container(id: String, name: String) -> Result<(), String> {
    let dock = connect_docker().await?;
    let opt = RenameContainerOptions { name };
    dock.rename_container(&id, opt).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_image(id: String, force: Option<bool>) -> Result<(), String> {
    let dock = connect_docker().await?;
    let opt = RemoveImageOptions {
        force: force.unwrap_or(false),
        ..Default::default()
    };
    dock.remove_image(&id, Some(opt), None).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn remove_volume(name: String, force: Option<bool>) -> Result<(), String> {
    let dock = connect_docker().await?;
    let opt = RemoveVolumeOptions { force: force.unwrap_or(false) };
    dock.remove_volume(&name, Some(opt)).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_network(name: String) -> Result<(), String> {
    let dock = connect_docker().await?;
    dock.remove_network(&name).await.map_err(|e| e.to_string())
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RunImageRequest {
    image: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    env: Option<Vec<String>>,
    #[serde(default)]
    ports: Option<Vec<String>>,
    #[serde(default)]
    cmd: Option<Vec<String>>,
}

#[tauri::command]
async fn run_image(req: RunImageRequest) -> Result<String, String> {
    let dock = connect_docker().await?;

    let mut exposed_ports: Vec<String> = Vec::new();
    let mut port_bindings = std::collections::HashMap::new();

    if let Some(ports) = &req.ports {
        for mapping in ports {
            let mapping = mapping.trim();
            if mapping.is_empty() {
                continue;
            }
            let (host, container) = match mapping.split_once(':') {
                Some((h, c)) => (h.trim().to_string(), c.trim().to_string()),
                None => {
                    let p = mapping.to_string();
                    (p.clone(), p)
                }
            };
            let key = format!("{container}/tcp");
            exposed_ports.push(key.clone());
            port_bindings.insert(
                key,
                Some(vec![PortBinding {
                    host_ip: None,
                    host_port: Some(host),
                }]),
            );
        }
    }

    let config = ContainerCreateBody {
        image: Some(req.image.clone()),
        env: req.env.clone(),
        cmd: req.cmd.clone(),
        exposed_ports: if exposed_ports.is_empty() {
            None
        } else {
            Some(exposed_ports)
        },
        host_config: if port_bindings.is_empty() {
            None
        } else {
            Some(HostConfig {
                port_bindings: Some(port_bindings),
                ..Default::default()
            })
        },
        ..Default::default()
    };

    let opt = CreateContainerOptions { name: req.name.clone(), ..Default::default() };
    let container = dock
        .create_container(Some(opt), config)
        .await
        .map_err(|e| e.to_string())?;

    dock.start_container(&container.id, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(container.id)
}

#[tauri::command]
async fn podman_action(id: String, action: String) -> Result<(), String> {
    let sub = match action.as_str() {
        "start" => "start",
        "stop" => "stop",
        "restart" => "restart",
        "remove" => "rm",
        _ => return Err(format!("Aksi pod tidak dikenal: {action}")),
    };

    let mut cmd = Command::new("podman");
    cmd.args(["pod", sub, &id]);
    if sub == "rm" {
        cmd.arg("--force");
    }

    let output = cmd.output().await.map_err(|e| format!("Gagal mengeksekusi podman: {e}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_containers,
            get_container,
            get_container_logs,
            get_images,
            get_image,
            get_image_history,
            get_image_filesystem,
            get_volumes,
            get_networks,
            get_pods,
            events_status,
            start_container,
            stop_container,
            restart_container,
            remove_container,
            rename_container,
            remove_image,
            remove_volume,
            remove_network,
            run_image,
            podman_action
        ])
        .setup(|app| {
            app.manage(events::StatusStore::default());

            let window = app
                .get_webview_window("main")
                .expect("Window 'main' tidak ditemukan");

            events::spawn(window);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn commands_read_from_podman_socket() {
        if let Err(err) = connect_docker().await {
            eprintln!("lewat: tidak ada socket Podman/Docker ({err})");
            return;
        }

        let results = [
            ("containers", get_containers().await.map(|list| list.len())),
            ("images", get_images().await.map(|list| list.len())),
            (
                "volumes",
                get_volumes()
                    .await
                    .map(|list| list.map(|list| list.len()).unwrap_or(0)),
            ),
            ("networks", get_networks().await.map(|list| list.len())),
        ];

        let mut failures = Vec::new();
        for (name, result) in results {
            match result {
                Ok(count) => eprintln!("{name}: {count}"),
                Err(err) => failures.push(format!("{name}: {err}")),
            }
        }

        assert!(
            failures.is_empty(),
            "perintah gagal membaca socket Podman: {failures:?}"
        );
    }
}
