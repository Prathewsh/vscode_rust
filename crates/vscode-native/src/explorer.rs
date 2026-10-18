use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct TreeItem {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub is_expanded: bool,
}

pub struct ExplorerState {
    pub root_path: Option<PathBuf>,
    pub expanded_dirs: HashSet<PathBuf>,
}

impl ExplorerState {
    pub fn new(root: Option<PathBuf>) -> Self {
        let mut state = Self {
            root_path: root.clone(),
            expanded_dirs: HashSet::new(),
        };
        if let Some(ref r) = root {
            state.expanded_dirs.insert(r.clone());
        }
        state
    }

    pub fn load_directory(&mut self, root: &Path) {
        self.root_path = Some(root.to_path_buf());
        self.expanded_dirs.clear();
        self.expanded_dirs.insert(root.to_path_buf());
    }

    pub fn toggle_expand(&mut self, path: &Path) {
        if self.expanded_dirs.contains(path) {
            self.expanded_dirs.remove(path);
        } else {
            self.expanded_dirs.insert(path.to_path_buf());
        }
    }

    pub fn get_visible_tree(&self) -> Vec<TreeItem> {
        let mut items = Vec::new();
        if let Some(ref root) = self.root_path {
            self.collect_children(root, 0, &mut items);
        }
        items
    }

    fn collect_children(&self, dir: &Path, depth: usize, out: &mut Vec<TreeItem>) {
        let mut entries = Vec::new();
        if let Ok(rd) = std::fs::read_dir(dir) {
            for entry in rd.flatten() {
                let path = entry.path();
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                if file_name.starts_with('.') && file_name != ".gitignore" {
                    continue;
                }
                if file_name == "target" || file_name == "node_modules" {
                    continue;
                }

                let is_dir = path.is_dir();
                entries.push((path, file_name, is_dir));
            }
        }

        entries.sort_by(|a, b| {
            if a.2 == b.2 {
                a.1.to_lowercase().cmp(&b.1.to_lowercase())
            } else if a.2 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        for (path, name, is_dir) in entries {
            let is_expanded = is_dir && self.expanded_dirs.contains(&path);
            out.push(TreeItem {
                path: path.clone(),
                name,
                is_dir,
                depth,
                is_expanded,
            });

            if is_dir && is_expanded {
                self.collect_children(&path, depth + 1, out);
            }
        }
    }

    pub fn create_file(&self, dir: &Path, name: &str) -> std::io::Result<PathBuf> {
        let target = dir.join(name);
        std::fs::File::create(&target)?;
        Ok(target)
    }

    pub fn create_dir(&self, dir: &Path, name: &str) -> std::io::Result<PathBuf> {
        let target = dir.join(name);
        std::fs::create_dir_all(&target)?;
        Ok(target)
    }

    #[allow(dead_code)]
    pub fn delete_path(&self, path: &Path) -> std::io::Result<()> {
        if path.is_dir() {
            std::fs::remove_dir_all(path)
        } else {
            std::fs::remove_file(path)
        }
    }

    #[allow(dead_code)]
    pub fn file_icon_type(name: &str, is_dir: bool) -> &'static str {
        if is_dir {
            "folder"
        } else if name.ends_with(".rs") {
            "rust"
        } else if name == "Cargo.toml" || name.ends_with(".toml") {
            "toml"
        } else if name.ends_with(".lock") {
            "lock"
        } else if name.ends_with(".json") {
            "json"
        } else if name.ends_with(".md") || name.ends_with(".txt") {
            "markdown"
        } else if name == ".gitignore" || name.contains("git") {
            "git"
        } else if name.ends_with(".ts") || name.ends_with(".tsx") {
            "typescript"
        } else if name.ends_with(".js") || name.ends_with(".jsx") {
            "javascript"
        } else if name.ends_with(".sh") || name.ends_with(".bash") || name.ends_with(".zsh") {
            "shell"
        } else if name.ends_with(".html") || name.ends_with(".css") {
            "web"
        } else if name.ends_with(".png") || name.ends_with(".jpg") || name.ends_with(".svg") {
            "image"
        } else {
            "file"
        }
    }
}


