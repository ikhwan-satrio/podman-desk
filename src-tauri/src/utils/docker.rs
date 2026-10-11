use bollard::Docker;

pub async fn connect_docker() -> Result<Docker, String> {
    // Rootless podman ($XDG_RUNTIME_DIR/podman/podman.sock / /run/user/$UID/podman/podman.sock),
    // lalu system podman (/run/podman/podman.sock), lalu docker socket.
    #[cfg(unix)]
    let candidates = vec![
        Docker::connect_with_podman_defaults(),
        Docker::connect_with_local_defaults(),
    ];

    #[cfg(not(unix))]
    let candidates = vec![Docker::connect_with_local_defaults()];

    let mut errors = Vec::new();

    for candidate in candidates {
        let dock = match candidate {
            Ok(dock) => dock,
            Err(err) => {
                errors.push(err.to_string());
                continue;
            }
        };

        match dock.ping().await {
            Ok(_) => return Ok(dock),
            Err(err) => errors.push(err.to_string()),
        }
    }

    Err(format!(
        "Tidak ada socket Podman/Docker yang merespons: {}",
        errors.join(" | ")
    ))
}
