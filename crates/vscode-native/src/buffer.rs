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
    pub text: String,
    pub rope: Rope,
    pub cursor_char: usize,
    pub selection_anchor: Option<usize>,
    pub requested_cursor_char: Option<usize>,
    pub is_dirty: bool,
    undo_stack: Vec<UndoEntry>,
    redo_stack: Vec<UndoEntry>,
}

#[allow(dead_code)]
impl EditorBuffer {
    pub fn empty() -> Self {
        let text = "// Welcome to VS Code (Native Rust Edition)\n// Interactive text editing, undo/redo, syntax highlighting, and integrated terminal.\n\nfn main() {\n    println!(\"Hello, native VS Code!\");\n}\n";
        Self {
            path: None,
            file_name: "Untitled-1".to_string(),
            text: text.to_string(),
            rope: Rope::from_str(text),
            cursor_char: 0,
            selection_anchor: None,
            requested_cursor_char: None,
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
            text: content.clone(),
            rope: Rope::from_str(&content),
            cursor_char: 0,
            selection_anchor: None,
            requested_cursor_char: None,
            is_dirty: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        })
    }

    pub fn sync_from_text(&mut self) {
        self.rope = Rope::from_str(&self.text);
    }

    pub fn sync_to_text(&mut self) {
        self.text = self.rope.to_string();
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.path {
            fs::write(path, &self.text)?;
            self.rope = Rope::from_str(&self.text);
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
            self.sync_to_text();
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
            self.sync_to_text();
        }
    }

    #[allow(dead_code)]
    pub fn insert_char(&mut self, ch: char) {
        self.delete_selection_if_any();
        self.push_undo();
        let idx = self.cursor_char.min(self.rope.len_chars());
        self.rope.insert_char(idx, ch);
        self.cursor_char = idx + 1;
        self.sync_to_text();
    }

    pub fn insert_str(&mut self, text: &str) {
        self.delete_selection_if_any();
        self.push_undo();
        let idx = self.cursor_char.min(self.rope.len_chars());
        self.rope.insert(idx, text);
        self.cursor_char = idx + text.chars().count();
        self.sync_to_text();
    }

    pub fn backspace(&mut self) {
        if self.delete_selection_if_any() {
            self.sync_to_text();
            return;
        }
        if self.cursor_char > 0 {
            self.push_undo();
            let idx = self.cursor_char - 1;
            self.rope.remove(idx..self.cursor_char);
            self.cursor_char = idx;
            self.sync_to_text();
        }
    }

    pub fn delete_forward(&mut self) {
        if self.delete_selection_if_any() {
            self.sync_to_text();
            return;
        }
        if self.cursor_char < self.rope.len_chars() {
            self.push_undo();
            self.rope.remove(self.cursor_char..self.cursor_char + 1);
            self.sync_to_text();
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

    pub fn select_all(&mut self) {
        self.selection_anchor = Some(0);
        self.cursor_char = self.rope.len_chars();
    }

    pub fn selected_text(&self) -> Option<String> {
        if let Some(anchor) = self.selection_anchor {
            if anchor != self.cursor_char {
                let start = anchor.min(self.cursor_char).min(self.rope.len_chars());
                let end = anchor.max(self.cursor_char).min(self.rope.len_chars());
                return Some(self.rope.slice(start..end).to_string());
            }
        }
        None
    }

    pub fn cut_selected_text(&mut self) -> Option<String> {
        let text = self.selected_text()?;
        self.delete_selection_if_any();
        Some(text)
    }

    pub fn insert_newline_with_indent(&mut self) {
        let cur_line = self.cursor_line();
        let line_content = self.line_str(cur_line).unwrap_or_default();
        let indent: String = line_content
            .chars()
            .take_while(|c| *c == ' ' || *c == '\t')
            .collect();
        let mut to_insert = String::from("\n");
        to_insert.push_str(&indent);
        self.insert_str(&to_insert);
    }

    pub fn go_to_line(&mut self, target_line_1_based: usize) {
        let line_0 = target_line_1_based.saturating_sub(1);
        let total = self.rope.len_lines();
        let actual = line_0.min(total.saturating_sub(1));
        self.cursor_char = self.rope.line_to_char(actual);
        self.requested_cursor_char = Some(self.cursor_char);
        self.selection_anchor = None;
    }

    pub fn set_cursor_from_line_col(&mut self, line: usize, col: usize) {
        if line < self.rope.len_lines() {
            let line_start = self.rope.line_to_char(line);
            let line_len = self.rope.line(line).len_chars().saturating_sub(1);
            self.cursor_char = (line_start + col.min(line_len)).min(self.rope.len_chars());
            self.requested_cursor_char = Some(self.cursor_char);
            self.selection_anchor = None;
        }
    }

    pub fn find_next(&mut self, query: &str, case_sensitive: bool) -> bool {
        if query.is_empty() || self.rope.len_chars() == 0 {
            return false;
        }
        let content = self.rope.to_string();
        let current_char = self.cursor_char.min(content.len());
        let q = if case_sensitive { query.to_string() } else { query.to_lowercase() };
        let h = if case_sensitive { content.clone() } else { content.to_lowercase() };

        // Search after current cursor, then wrap around
        let after_slice = &h[current_char..];
        let found = after_slice.find(&q).map(|idx| current_char + idx)
            .or_else(|| h[..current_char].find(&q));

        if let Some(byte_pos) = found {
            let char_pos = content[..byte_pos].chars().count();
            let query_chars = query.chars().count();
            self.selection_anchor = Some(char_pos);
            self.cursor_char = char_pos + query_chars;
            self.requested_cursor_char = Some(self.cursor_char);
            true
        } else {
            false
        }
    }

    pub fn find_prev(&mut self, query: &str, case_sensitive: bool) -> bool {
        if query.is_empty() || self.rope.len_chars() == 0 {
            return false;
        }
        let content = self.rope.to_string();
        let current_char = self.selection_anchor.unwrap_or(self.cursor_char).min(content.len());
        let q = if case_sensitive { query.to_string() } else { query.to_lowercase() };
        let h = if case_sensitive { content.clone() } else { content.to_lowercase() };

        // Search before current position, then wrap to end
        let before_slice = &h[..current_char];
        let found = before_slice.rfind(&q)
            .or_else(|| h[current_char..].rfind(&q).map(|idx| current_char + idx));

        if let Some(byte_pos) = found {
            let char_pos = content[..byte_pos].chars().count();
            let query_chars = query.chars().count();
            self.selection_anchor = Some(char_pos);
            self.cursor_char = char_pos + query_chars;
            self.requested_cursor_char = Some(self.cursor_char);
            true
        } else {
            false
        }
    }

    pub fn count_matches(&self, query: &str, case_sensitive: bool) -> usize {
        if query.is_empty() {
            return 0;
        }
        let content = self.rope.to_string();
        let q = if case_sensitive { query.to_string() } else { query.to_lowercase() };
        let h = if case_sensitive { content } else { content.to_lowercase() };
        h.match_indices(&q).count()
    }
}

