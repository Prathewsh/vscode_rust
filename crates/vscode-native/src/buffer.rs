use ropey::Rope;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct UndoEntry {
    rope: Rope,
    cursor_char: usize,
}

pub struct EditorBuffer {
    pub path: Option<PathBuf>,
    pub file_name: String,
    pub rope: Rope,
    pub cursor_char: usize,
    pub selection_anchor: Option<usize>,
    pub is_dirty: bool,
    undo_stack: Vec<UndoEntry>,
    redo_stack: Vec<UndoEntry>,
}

impl EditorBuffer {
    pub fn empty() -> Self {
        let text = "// Welcome to VS Code (Native Rust Edition)\n// Interactive text editing, undo/redo, syntax highlighting, and integrated terminal.\n\nfn main() {\n    println!(\"Hello, native VS Code!\");\n}\n";
        Self {
            path: None,
            file_name: "Untitled-1".to_string(),
            rope: Rope::from_str(text),
            cursor_char: 0,
            selection_anchor: None,
            is_dirty: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn from_file(path: &Path) -> std::io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();

        Ok(Self {
            path: Some(path.to_path_buf()),
            file_name,
            rope: Rope::from_str(&content),
            cursor_char: 0,
            selection_anchor: None,
            is_dirty: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.path {
            let mut file = fs::File::create(path)?;
            self.rope.write_to(&mut file)?;
            self.is_dirty = false;
        }
        Ok(())
    }

    fn push_undo(&mut self) {
        self.undo_stack.push(UndoEntry {
            rope: self.rope.clone(),
            cursor_char: self.cursor_char,
        });
        if self.undo_stack.len() > 100 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
        self.is_dirty = true;
    }

    pub fn undo(&mut self) {
        if let Some(entry) = self.undo_stack.pop() {
            self.redo_stack.push(UndoEntry {
                rope: self.rope.clone(),
                cursor_char: self.cursor_char,
            });
            self.rope = entry.rope;
            self.cursor_char = entry.cursor_char.min(self.rope.len_chars());
            self.selection_anchor = None;
        }
    }

    pub fn redo(&mut self) {
        if let Some(entry) = self.redo_stack.pop() {
            self.undo_stack.push(UndoEntry {
                rope: self.rope.clone(),
                cursor_char: self.cursor_char,
            });
            self.rope = entry.rope;
            self.cursor_char = entry.cursor_char.min(self.rope.len_chars());
            self.selection_anchor = None;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.delete_selection_if_any();
        self.push_undo();
        let idx = self.cursor_char.min(self.rope.len_chars());
        self.rope.insert_char(idx, ch);
        self.cursor_char = idx + 1;
    }

    pub fn insert_str(&mut self, text: &str) {
        self.delete_selection_if_any();
        self.push_undo();
        let idx = self.cursor_char.min(self.rope.len_chars());
        self.rope.insert(idx, text);
        self.cursor_char = idx + text.chars().count();
    }

    pub fn backspace(&mut self) {
        if self.delete_selection_if_any() {
            return;
        }
        if self.cursor_char > 0 {
            self.push_undo();
            let idx = self.cursor_char - 1;
            self.rope.remove(idx..self.cursor_char);
            self.cursor_char = idx;
        }
    }

    pub fn delete_forward(&mut self) {
        if self.delete_selection_if_any() {
            return;
        }
        if self.cursor_char < self.rope.len_chars() {
            self.push_undo();
            self.rope.remove(self.cursor_char..self.cursor_char + 1);
        }
    }

    pub fn delete_selection_if_any(&mut self) -> bool {
        if let Some(anchor) = self.selection_anchor {
            if anchor != self.cursor_char {
                self.push_undo();
                let start = anchor.min(self.cursor_char);
                let end = anchor.max(self.cursor_char);
                self.rope.remove(start..end);
                self.cursor_char = start;
                self.selection_anchor = None;
                return true;
            }
        }
        self.selection_anchor = None;
        false
    }

    pub fn move_cursor_left(&mut self, select: bool) {
        if select && self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor_char);
        } else if !select {
            self.selection_anchor = None;
        }
        if self.cursor_char > 0 {
            self.cursor_char -= 1;
        }
    }

    pub fn move_cursor_right(&mut self, select: bool) {
        if select && self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor_char);
        } else if !select {
            self.selection_anchor = None;
        }
        if self.cursor_char < self.rope.len_chars() {
            self.cursor_char += 1;
        }
    }

    pub fn move_cursor_up(&mut self, select: bool) {
        if select && self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor_char);
        } else if !select {
            self.selection_anchor = None;
        }
        let line = self.cursor_line();
        if line > 0 {
            let col = self.cursor_col();
            let target_line = line - 1;
            let target_line_len = self.rope.line(target_line).len_chars().saturating_sub(1);
            let target_col = col.min(target_line_len);
            let target_char = self.rope.line_to_char(target_line) + target_col;
            self.cursor_char = target_char;
        }
    }

    pub fn move_cursor_down(&mut self, select: bool) {
        if select && self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor_char);
        } else if !select {
            self.selection_anchor = None;
        }
        let line = self.cursor_line();
        if line + 1 < self.rope.len_lines() {
            let col = self.cursor_col();
            let target_line = line + 1;
            let target_line_len = self.rope.line(target_line).len_chars().saturating_sub(1);
            let target_col = col.min(target_line_len);
            let target_char = self.rope.line_to_char(target_line) + target_col;
            self.cursor_char = target_char.min(self.rope.len_chars());
        }
    }

    pub fn cursor_line(&self) -> usize {
        let idx = self.cursor_char.min(self.rope.len_chars());
        self.rope.char_to_line(idx)
    }

    pub fn cursor_col(&self) -> usize {
        let idx = self.cursor_char.min(self.rope.len_chars());
        let line = self.rope.char_to_line(idx);
        let line_start = self.rope.line_to_char(line);
        idx.saturating_sub(line_start)
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn line_str(&self, idx: usize) -> Option<String> {
        if idx < self.rope.len_lines() {
            Some(self.rope.line(idx).to_string())
        } else {
            None
        }
    }

    pub fn get_extension(&self) -> &str {
        self.path
            .as_ref()
            .and_then(|p| p.extension())
            .and_then(|e| e.to_str())
            .unwrap_or("rs")
    }
}
