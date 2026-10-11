pub mod docker;

#[cfg(unix)]
pub mod events;

/// Live event hanya tersedia di unix (socket Podman/Docker).
#[cfg(not(unix))]
pub mod events {
    use serde::Serialize;

    #[derive(Clone, Debug, Default, Serialize)]
    pub struct EventsStatus {
        pub connected: bool,
        pub socket: Option<String>,
        pub detail: Option<String>,
    }

    #[derive(Default)]
    pub struct StatusStore;

    pub fn current(_: &tauri::WebviewWindow) -> EventsStatus {
        EventsStatus::default()
    }

    pub fn spawn(_: tauri::WebviewWindow) {
        eprintln!("live event Podman hanya didukung di unix");
    }
}
