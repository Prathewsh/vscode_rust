use std::path::{Path, PathBuf};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PaletteMode {
    Command,
    QuickOpen,
    GoToLine,
}

#[derive(Clone, Debug)]
pub struct CommandItem {
    pub id: &'static str,
    pub title: &'static str,
    pub category: &'static str,
    pub keybinding: &'static str,
}

pub fn get_all_commands() -> Vec<CommandItem> {
    vec![
        CommandItem { id: "file.new", title: "New File", category: "File", keybinding: "Ctrl+N" },
        CommandItem { id: "file.open_file", title: "Open File...", category: "File", keybinding: "Ctrl+O" },
        CommandItem { id: "file.open_folder", title: "Open Folder...", category: "File", keybinding: "Ctrl+K Ctrl+O" },
        CommandItem { id: "file.save", title: "Save File", category: "File", keybinding: "Ctrl+S" },
        CommandItem { id: "file.close_tab", title: "Close Active Editor", category: "File", keybinding: "Ctrl+W" },
        CommandItem { id: "file.close_all", title: "Close All Editors", category: "File", keybinding: "" },
        CommandItem { id: "view.toggle_terminal", title: "Toggle Integrated Terminal", category: "View", keybinding: "Ctrl+`" },
        CommandItem { id: "view.toggle_sidebar", title: "Toggle Primary Sidebar", category: "View", keybinding: "Ctrl+B" },
        CommandItem { id: "view.explorer", title: "Show Explorer", category: "View", keybinding: "Ctrl+Shift+E" },
        CommandItem { id: "view.search", title: "Show Search", category: "View", keybinding: "Ctrl+Shift+F" },
        CommandItem { id: "view.source_control", title: "Show Source Control", category: "View", keybinding: "Ctrl+Shift+G" },
        CommandItem { id: "view.settings", title: "Open Settings", category: "View", keybinding: "Ctrl+," },
        CommandItem { id: "edit.find", title: "Find in Buffer", category: "Edit", keybinding: "Ctrl+F" },
        CommandItem { id: "edit.goto_line", title: "Go to Line...", category: "Edit", keybinding: "Ctrl+G" },
        CommandItem { id: "edit.undo", title: "Undo", category: "Edit", keybinding: "Ctrl+Z" },
        CommandItem { id: "edit.redo", title: "Redo", category: "Edit", keybinding: "Ctrl+Y" },
        CommandItem { id: "edit.select_all", title: "Select All", category: "Edit", keybinding: "Ctrl+A" },
        CommandItem { id: "terminal.clear", title: "Clear Terminal Output", category: "Terminal", keybinding: "" },
        CommandItem { id: "terminal.cargo_check", title: "Run: cargo check", category: "Terminal", keybinding: "" },
        CommandItem { id: "terminal.cargo_test", title: "Run: cargo test", category: "Terminal", keybinding: "" },
        CommandItem { id: "terminal.cargo_run", title: "Run: cargo run", category: "Terminal", keybinding: "" },
        CommandItem { id: "theme.dark", title: "Color Theme: Dark+ (Default)", category: "Preferences", keybinding: "" },
        CommandItem { id: "theme.monokai", title: "Color Theme: Monokai", category: "Preferences", keybinding: "" },
        CommandItem { id: "theme.solarized", title: "Color Theme: Solarized Dark", category: "Preferences", keybinding: "" },
        CommandItem { id: "theme.github", title: "Color Theme: GitHub Dark", category: "Preferences", keybinding: "" },
        CommandItem { id: "theme.light", title: "Color Theme: Light+", category: "Preferences", keybinding: "" },
    ]
}

#[derive(Clone, Debug)]
pub struct SearchHit {
    pub path: PathBuf,
    pub relative_path: String,
    pub line_num: usize,
    pub line_content: String,
}

pub struct SearchState {
    pub query: String,
    pub replace_query: String,
    pub case_sensitive: bool,
    pub match_whole_word: bool,
    pub use_regex: bool,
    pub preserve_case: bool,
    pub show_details: bool,
    pub files_to_include: String,
    pub files_to_exclude: String,
    pub hits: Vec<SearchHit>,
    pub replace_message: String,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            replace_query: String::new(),
            case_sensitive: false,
            match_whole_word: false,
            use_regex: false,
            preserve_case: false,
            show_details: false,
            files_to_include: String::new(),
            files_to_exclude: String::new(),
            hits: Vec::new(),
            replace_message: String::new(),
        }
    }

    pub fn execute_search(&mut self, root: &Path) {
        self.hits.clear();
        self.replace_message.clear();
        let trimmed = self.query.trim();
        if trimmed.is_empty() {
            return;
        }

        let regex_opt = if self.use_regex {
            let mut pattern = trimmed.to_string();
            if self.match_whole_word {
                pattern = format!(r"\b{}\b", pattern);
            }
            fancy_regex::RegexBuilder::new(&pattern)
                .case_insensitive(!self.case_sensitive)
                .build()
                .ok()
        } else {
            None
        };

        let q = if self.case_sensitive {
            trimmed.to_string()
        } else {
            trimmed.to_lowercase()
        };

        let inc = self.files_to_include.trim().to_lowercase();
        let exc = self.files_to_exclude.trim().to_lowercase();

        for entry in walkdir::WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_str().unwrap_or("");
                name != ".git" && name != "target" && name != "node_modules" && !name.starts_with('.')
            })
            .flatten()
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().to_string();
            let rel_lower = rel.to_lowercase();

            if !inc.is_empty() && !rel_lower.contains(&inc) {
                continue;
            }
            if !exc.is_empty() && rel_lower.contains(&exc) {
                continue;
            }

            if let Ok(meta) = entry.metadata() {
                if meta.len() > 1_000_000 {
                    continue;
                }
            }

            if let Ok(content) = std::fs::read_to_string(path) {
                for (idx, line) in content.lines().enumerate() {
                    let is_match = if let Some(ref re) = regex_opt {
                        re.is_match(line).unwrap_or(false)
                    } else if self.match_whole_word {
                        let hay = if self.case_sensitive { line } else { &line.to_lowercase() };
                        hay.split(|c: char| !c.is_alphanumeric() && c != '_').any(|w| w == q)
                    } else {
                        let hay = if self.case_sensitive { line.to_string() } else { line.to_lowercase() };
                        hay.contains(&q)
                    };

                    if is_match {
                        self.hits.push(SearchHit {
                            path: path.to_path_buf(),
                            relative_path: rel.clone(),
                            line_num: idx + 1,
                            line_content: line.trim().to_string(),
                        });
                        if self.hits.len() >= 300 {
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn execute_replace_all(&mut self, root: &Path) -> usize {
        if self.query.trim().is_empty() {
            return 0;
        }

        let mut unique_paths = std::collections::HashSet::new();
        for hit in &self.hits {
            unique_paths.insert(hit.path.clone());
        }

        let mut replacements_count = 0;
        for path in unique_paths {
            if let Ok(content) = std::fs::read_to_string(&path) {
                let new_content = if self.case_sensitive {
                    content.replace(&self.query, &self.replace_query)
                } else {
                    let mut result = String::with_capacity(content.len());
                    let mut last = 0;
                    let q_lower = self.query.to_lowercase();
                    let content_lower = content.to_lowercase();
                    for (start, _) in content_lower.match_indices(&q_lower) {
                        result.push_str(&content[last..start]);
                        result.push_str(&self.replace_query);
                        last = start + self.query.len();
                        replacements_count += 1;
                    }
                    result.push_str(&content[last..]);
                    result
                };

                if new_content != content {
                    let _ = std::fs::write(&path, new_content);
                }
            }
        }

        self.execute_search(root);
        self.replace_message = format!("Replaced occurrences across files");
        replacements_count
    }
}


#[derive(Clone, Debug)]
pub struct GitFileChange {
    pub status: String,
    pub path: PathBuf,
    pub display_name: String,
}

pub struct GitState {
    pub branch: String,
    pub changes: Vec<GitFileChange>,
    pub commit_message: String,
    pub status_feedback: String,
}

impl GitState {
    pub fn new() -> Self {
        Self {
            branch: "main".to_string(),
            changes: Vec::new(),
            commit_message: String::new(),
            status_feedback: String::new(),
        }
    }

    pub fn refresh(&mut self, root: Option<&Path>) {
        let Some(root) = root else { return; };

        if let Ok(output) = std::process::Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(root)
            .output()
        {
            if output.status.success() {
                let b = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !b.is_empty() {
                    self.branch = b;
                }
            }
        }

        self.changes.clear();
        if let Ok(output) = std::process::Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(root)
            .output()
        {
            if output.status.success() {
                let out = String::from_utf8_lossy(&output.stdout);
                for line in out.lines() {
                    if line.len() > 3 {
                        let status = line[..2].trim().to_string();
                        let file_part = line[3..].trim();
                        let target_path = root.join(file_part);
                        self.changes.push(GitFileChange {
                            status,
                            path: target_path,
                            display_name: file_part.to_string(),
                        });
                    }
                }
            }
        }
    }

    pub fn commit(&mut self, root: Option<&Path>) -> bool {
        let Some(root) = root else { return false; };
        if self.commit_message.trim().is_empty() {
            self.status_feedback = "Please enter a commit message".to_string();
            return false;
        }

        let _ = std::process::Command::new("git")
            .args(["add", "-A"])
            .current_dir(root)
            .output();

        match std::process::Command::new("git")
            .args(["commit", "-m", &self.commit_message])
            .current_dir(root)
            .output()
        {
            Ok(res) => {
                if res.status.success() {
                    self.status_feedback = "Committed successfully!".to_string();
                    self.commit_message.clear();
                    self.refresh(Some(root));
                    true
                } else {
                    let err = String::from_utf8_lossy(&res.stderr);
                    self.status_feedback = format!("Commit error: {}", err.trim());
                    false
                }
            }
            Err(e) => {
                self.status_feedback = format!("Failed to commit: {}", e);
                false
            }
        }
    }
}
