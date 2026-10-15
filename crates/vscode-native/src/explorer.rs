use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct FileNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
}

pub struct ExplorerState {
    pub root_path: Option<PathBuf>,
    pub entries: Vec<FileNode>,
}

impl ExplorerState {
    pub fn new(root: Option<PathBuf>) -> Self {
        let mut explorer = Self {
            root_path: root.clone(),
            entries: Vec::new(),
        };
        if let Some(r) = root {
            explorer.load_directory(&r);
        }
        explorer
    }

    pub fn load_directory(&mut self, root: &Path) {
        self.root_path = Some(root.to_path_buf());
        self.entries = Self::read_dir_shallow(root);
    }

    fn read_dir_shallow(dir: &Path) -> Vec<FileNode> {
        let mut list = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                if file_name.starts_with('.') && file_name != ".gitignore" {
                    continue;
                }

                let is_dir = path.is_dir();
                list.push(FileNode {
                    path,
                    name: file_name,
                    is_dir,
                });
            }
        }
        list.sort_by(|a, b| {
            if a.is_dir == b.is_dir {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            } else if a.is_dir {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        list
    }
}

