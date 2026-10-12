use std::collections::HashMap;
use std::io::{Seek, Write};

use bollard::models::ContainerCreateBody;
use bollard::query_parameters::{CreateContainerOptions, RemoveContainerOptions};
use bollard::Docker;
use futures_util::StreamExt;
use serde::Serialize;
use tar::EntryType;

pub const ENTRY_LIMIT: usize = 30_000;

#[derive(Debug, Clone, Serialize)]
pub struct FsNode {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub kind: &'static str,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<FsNode>,
}

#[derive(Debug, Serialize)]
pub struct ImageFs {
    pub root: FsNode,
    pub file_count: usize,
    pub dir_count: usize,
    pub truncated: bool,
}

/// Buat container sementara dari image, export filesystem-nya sebagai tar,
/// lalu susun jadi tree. Container selalu dihapus, baik sukses maupun gagal.
pub async fn read_image_fs(dock: &Docker, image: &str) -> Result<ImageFs, String> {
    let name = format!(
        "podman-desk-fs-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    );

    let container = dock
        .create_container(
            Some(CreateContainerOptions {
                name: Some(name),
                platform: String::new(),
            }),
            ContainerCreateBody {
                image: Some(image.to_string()),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| format!("gagal membuat container sementara: {e}"))?;

    let result = export_tree(dock, &container.id).await;

    let _ = dock
        .remove_container(
            &container.id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await;

    result
}

async fn export_tree(dock: &Docker, container_id: &str) -> Result<ImageFs, String> {
    let mut temp = tempfile::tempfile().map_err(|e| e.to_string())?;
    let mut stream = dock.export_container(container_id);

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        temp.write_all(&chunk).map_err(|e| e.to_string())?;
    }

    temp.flush().map_err(|e| e.to_string())?;
    temp.rewind().map_err(|e| e.to_string())?;

    tokio::task::spawn_blocking(move || parse_tar(temp))
        .await
        .map_err(|e| e.to_string())?
}

fn parse_tar(file: std::fs::File) -> Result<ImageFs, String> {
    let mut archive = tar::Archive::new(file);
    let mut by_path: HashMap<String, usize> = HashMap::new();
    by_path.insert("/".into(), 0);

    let mut nodes: Vec<Node> = vec![Node::dir("/".into(), "/".into())];
    let mut file_count = 0usize;
    let mut dir_count = 1usize;
    let mut truncated = false;

    let entries = archive.entries().map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;

        if nodes.len() >= ENTRY_LIMIT {
            truncated = true;
            break;
        }

        let raw_path = entry
            .path()
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .into_owned();
        let path = normalize(&raw_path);
        if path.is_empty() {
            continue;
        }

        let entry_type = entry.header().entry_type();
        let kind: &'static str = match entry_type {
            EntryType::Directory => "dir",
            EntryType::Symlink | EntryType::Link => "symlink",
            EntryType::Regular | EntryType::Continuous => "file",
            _ => "other",
        };
        let size = if kind == "file" {
            entry.header().size().unwrap_or(0)
        } else {
            0
        };
        let link = if kind == "symlink" {
            entry
                .link_name()
                .ok()
                .flatten()
                .map(|name| name.to_string_lossy().into_owned())
        } else {
            None
        };

        // Buat semua parent yang belum ada, lalu node terakhir.
        let parts: Vec<&str> = path.split('/').collect();
        let mut current = String::new();
        let mut parent_idx = 0usize;

        for (index, part) in parts.iter().enumerate() {
            if index > 0 {
                current.push('/');
            }
            current.push_str(part);

            let is_last = index == parts.len() - 1;
            if let Some(&existing) = by_path.get(&current) {
                if is_last {
                    // Duplikat (mis. tar punya dua entry sama) — lewati.
                    break;
                }
                parent_idx = existing;
                continue;
            }

            let node_kind = if is_last { kind } else { "dir" };
            if node_kind == "dir" {
                dir_count += 1;
            } else {
                file_count += 1;
            }

            let node = Node {
                name: part.to_string(),
                path: current.clone(),
                kind: node_kind,
                size: if is_last { size } else { 0 },
                link: if is_last { link.clone() } else { None },
                children: Vec::new(),
            };

            nodes.push(node);
            let idx = nodes.len() - 1;
            by_path.insert(current.clone(), idx);
            nodes[parent_idx].children.push(idx);
            parent_idx = idx;
        }
    }

    // Sort: folder dulu, lalu nama A→Z, rekursif.
    sort_children(&mut nodes, 0);

    let root_node = to_fs_node(0, &nodes);
    let dir_count = dir_count.max(1);

    Ok(ImageFs {
        root: root_node,
        file_count,
        dir_count,
        truncated,
    })
}

struct Node {
    name: String,
    path: String,
    kind: &'static str,
    size: u64,
    link: Option<String>,
    children: Vec<usize>,
}

impl Node {
    fn dir(name: String, path: String) -> Self {
        Node {
            name,
            path,
            kind: "dir",
            size: 0,
            link: None,
            children: Vec::new(),
        }
    }
}

fn normalize(path: &str) -> String {
    path.trim_start_matches("./")
        .trim_start_matches('/')
        .trim_end_matches('/')
        .to_string()
}

fn sort_children(nodes: &mut [Node], index: usize) {
    let children = std::mem::take(&mut nodes[index].children);
    let mut children = children;

    children.sort_by(|&a, &b| {
        let dir_a = nodes[a].kind == "dir";
        let dir_b = nodes[b].kind == "dir";
        dir_b
            .cmp(&dir_a)
            .then_with(|| nodes[a].name.to_lowercase().cmp(&nodes[b].name.to_lowercase()))
    });

    for &child in &children {
        sort_children(nodes, child);
    }

    nodes[index].children = children;
}

fn to_fs_node(index: usize, nodes: &[Node]) -> FsNode {
    let node = &nodes[index];
    FsNode {
        name: node.name.clone(),
        path: node.path.clone(),
        kind: node.kind,
        size: node.size,
        link: node.link.clone(),
        children: node
            .children
            .iter()
            .map(|&child| to_fs_node(child, nodes))
            .collect(),
    }
}
