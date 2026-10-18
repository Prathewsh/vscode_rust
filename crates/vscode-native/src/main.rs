mod buffer;
mod explorer;
mod icons;
mod palette;
mod syntax;
mod terminal;
mod theme;

use buffer::EditorBuffer;
use explorer::ExplorerState;
use icons::MaterialIcon;
use palette::{get_all_commands, CommandItem, GitState, PaletteMode, SearchState};
use syntax::SyntaxHighlighter;
use terminal::TerminalManager;
use theme::*;

use eframe::egui::{self, Color32, FontId, Key, Pos2, Rect, RichText, Vec2};
use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq, Clone, Copy)]
enum ActivityBarTab {
    Explorer,
    Search,
    SourceControl,
    RunAndDebug,
    Remote,
    Extensions,
    Packages,
    Settings,
}

struct VsCodeApp {
    theme: ThemeColors,
    active_tab: ActivityBarTab,
    sidebar_open: bool,
    explorer: ExplorerState,
    open_buffers: Vec<EditorBuffer>,
    active_buffer_idx: usize,
    syntax_highlighter: SyntaxHighlighter,
    terminal_mgr: TerminalManager,
    terminal_open: bool,
    terminal_maximized: bool,
    terminal_tab: String,
    terminal_input: String,
    status_msg: String,

    // Command palette / Quick open
    palette_open: bool,
    palette_mode: PaletteMode,
    palette_query: String,
    palette_selected_idx: usize,
    cached_files: Vec<(String, PathBuf)>,

    // Find widget
    find_open: bool,
    find_query: String,
    find_case_sensitive: bool,

    // Search and Git
    search_state: SearchState,
    search_show_replace: bool,
    git_state: GitState,

    // File creation modal / inline
    new_item_dialog_open: bool,
    new_item_is_dir: bool,
    new_item_name: String,

    // Editor settings
    editor_font_size: f32,
}

impl VsCodeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = ThemeColors::from_preset(ThemePreset::Antigravity);
        apply_theme(&cc.egui_ctx, &theme);

        let current_dir = std::env::current_dir().ok();
        let explorer = ExplorerState::new(current_dir.clone());
        let initial_buffer = EditorBuffer::empty();
        let terminal_mgr = TerminalManager::new();

        let mut git_state = GitState::new();
        git_state.refresh(current_dir.as_deref());

        let mut app = Self {
            theme,
            active_tab: ActivityBarTab::Search,
            sidebar_open: true,
            explorer,
            open_buffers: vec![initial_buffer],
            active_buffer_idx: 0,
            syntax_highlighter: SyntaxHighlighter::new(),
            terminal_mgr,
            terminal_open: true,
            terminal_maximized: false,
            terminal_tab: "Terminal".to_string(),
            terminal_input: String::new(),
            status_msg: "Antigravity Code Ready".to_string(),

            palette_open: false,
            palette_mode: PaletteMode::Command,
            palette_query: String::new(),
            palette_selected_idx: 0,
            cached_files: Vec::new(),

            find_open: false,
            find_query: String::new(),
            find_case_sensitive: false,

            search_state: SearchState::new(),
            search_show_replace: true,
            git_state,

            new_item_dialog_open: false,
            new_item_is_dir: false,
            new_item_name: String::new(),

            editor_font_size: 13.0,
        };

        app.refresh_cached_files();
        app
    }

    fn send_to_active_terminal(&mut self, text: &str) {
        if let Some(session) = self.terminal_mgr.active_session() {
            session.terminal.send_input(text);
        }
    }

    fn refresh_cached_files(&mut self) {
        self.cached_files.clear();
        if let Some(ref root) = self.explorer.root_path {
            for entry in walkdir::WalkDir::new(root)
                .into_iter()
                .filter_entry(|e| {
                    let name = e.file_name().to_str().unwrap_or("");
                    name != ".git" && name != "target" && name != "node_modules" && !name.starts_with('.')
                })
                .flatten()
            {
                if entry.file_type().is_file() {
                    let path = entry.path().to_path_buf();
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                    self.cached_files.push((name, path));
                    if self.cached_files.len() >= 500 {
                        break;
                    }
                }
            }
        }
    }

    fn open_file(&mut self, path: &Path) {
        if let Some(idx) = self.open_buffers.iter().position(|b| b.path.as_deref() == Some(path)) {
            self.active_buffer_idx = idx;
            return;
        }

        match EditorBuffer::from_file(path) {
            Ok(buf) => {
                self.open_buffers.push(buf);
                self.active_buffer_idx = self.open_buffers.len() - 1;
                self.status_msg = format!("Opened: {}", path.display());
            }
            Err(e) => {
                self.status_msg = format!("Failed to open {}: {}", path.display(), e);
            }
        }
    }

    fn new_untitled_file(&mut self) {
        let count = self.open_buffers.len() + 1;
        let mut buf = EditorBuffer::empty();
        buf.file_name = format!("Untitled-{}", count);
        buf.is_dirty = true;
        self.open_buffers.push(buf);
        self.active_buffer_idx = self.open_buffers.len() - 1;
        self.status_msg = "Created new file".to_string();
    }

    fn close_tab(&mut self, idx: usize) {
        if idx < self.open_buffers.len() {
            self.open_buffers.remove(idx);
            if self.active_buffer_idx >= self.open_buffers.len() {
                self.active_buffer_idx = self.open_buffers.len().saturating_sub(1);
            }
        }
    }

    fn set_theme_preset(&mut self, ctx: &egui::Context, preset: ThemePreset) {
        self.theme = ThemeColors::from_preset(preset);
        self.syntax_highlighter.set_theme(self.theme.syntect_theme);
        apply_theme(ctx, &self.theme);
        self.status_msg = format!("Theme changed to: {}", self.theme.name);
    }

    fn execute_command(&mut self, ctx: &egui::Context, cmd_id: &str) {
        self.palette_open = false;
        match cmd_id {
            "file.new" => self.new_untitled_file(),
            "file.open_file" => {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.open_file(&path);
                }
            }
            "file.open_folder" => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.explorer.load_directory(&path);
                    self.git_state.refresh(Some(&path));
                    self.refresh_cached_files();
                }
            }
            "file.save" => {
                if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    if buf.path.is_none() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            buf.path = Some(path.clone());
                            buf.file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("Untitled").to_string();
                        }
                    }
                    let _ = buf.save();
                    self.status_msg = format!("Saved: {}", buf.file_name);
                }
            }
            "file.close_tab" => {
                self.close_tab(self.active_buffer_idx);
            }
            "file.close_all" => {
                self.open_buffers.clear();
                self.active_buffer_idx = 0;
            }
            "view.toggle_terminal" => self.terminal_open = !self.terminal_open,
            "view.toggle_sidebar" => self.sidebar_open = !self.sidebar_open,
            "view.explorer" => {
                self.sidebar_open = true;
                self.active_tab = ActivityBarTab::Explorer;
            }
            "view.search" => {
                self.sidebar_open = true;
                self.active_tab = ActivityBarTab::Search;
            }
            "view.source_control" => {
                self.sidebar_open = true;
                self.active_tab = ActivityBarTab::SourceControl;
                self.git_state.refresh(self.explorer.root_path.as_deref());
            }
            "view.settings" => {
                self.sidebar_open = true;
                self.active_tab = ActivityBarTab::Settings;
            }
            "edit.find" => {
                self.find_open = true;
            }
            "edit.goto_line" => {
                self.palette_mode = PaletteMode::GoToLine;
                self.palette_query = ":".to_string();
                self.palette_open = true;
            }
            "edit.undo" => {
                if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    buf.undo();
                }
            }
            "edit.redo" => {
                if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    buf.redo();
                }
            }
            "edit.select_all" => {
                if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    buf.select_all();
                }
            }
            "terminal.clear" => {
                if let Some(session) = self.terminal_mgr.active_session() {
                    session.terminal.clear();
                }
            }
            "terminal.cargo_check" => {
                self.send_to_active_terminal("cargo check\n");
                self.terminal_open = true;
            }
            "terminal.cargo_test" => {
                self.send_to_active_terminal("cargo test\n");
                self.terminal_open = true;
            }
            "terminal.cargo_run" => {
                self.send_to_active_terminal("cargo run\n");
                self.terminal_open = true;
            }
            "theme.dark" => self.set_theme_preset(ctx, ThemePreset::Antigravity),
            "theme.monokai" => self.set_theme_preset(ctx, ThemePreset::Monokai),
            "theme.solarized" => self.set_theme_preset(ctx, ThemePreset::SolarizedDark),
            "theme.github" => self.set_theme_preset(ctx, ThemePreset::GitHubDark),
            "theme.light" => self.set_theme_preset(ctx, ThemePreset::LightPlus),
            _ => {}
        }
    }

    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| i.clone());

        // Escape cancels palettes
        if input.key_pressed(Key::Escape) {
            if self.palette_open {
                self.palette_open = false;
                return;
            }
            if self.find_open {
                self.find_open = false;
                return;
            }
            if self.new_item_dialog_open {
                self.new_item_dialog_open = false;
                return;
            }
        }

        // Palette keyboard navigation
        if self.palette_open {
            if input.key_pressed(Key::ArrowDown) {
                self.palette_selected_idx = self.palette_selected_idx.saturating_add(1);
            } else if input.key_pressed(Key::ArrowUp) {
                self.palette_selected_idx = self.palette_selected_idx.saturating_sub(1);
            }
            return;
        }

        let ctrl_or_cmd = input.modifiers.command || input.modifiers.ctrl;

        // Shortcuts
        if ctrl_or_cmd {
            if input.key_pressed(Key::P) {
                if input.modifiers.shift {
                    self.palette_mode = PaletteMode::Command;
                    self.palette_query = "> ".to_string();
                } else {
                    self.palette_mode = PaletteMode::QuickOpen;
                    self.palette_query.clear();
                    if self.cached_files.is_empty() {
                        self.refresh_cached_files();
                    }
                }
                self.palette_selected_idx = 0;
                self.palette_open = true;
                return;
            } else if input.key_pressed(Key::F) {
                self.find_open = !self.find_open;
                return;
            } else if input.key_pressed(Key::G) {
                self.palette_mode = PaletteMode::GoToLine;
                self.palette_query = ":".to_string();
                self.palette_selected_idx = 0;
                self.palette_open = true;
                return;
            } else if input.key_pressed(Key::N) {
                self.new_untitled_file();
                return;
            } else if input.key_pressed(Key::W) {
                self.close_tab(self.active_buffer_idx);
                return;
            } else if input.key_pressed(Key::B) {
                self.sidebar_open = !self.sidebar_open;
                return;
            } else if input.key_pressed(Key::Backtick) {
                self.terminal_open = !self.terminal_open;
                return;
            } else if input.key_pressed(Key::S) {
                self.execute_command(ctx, "file.save");
                return;
            } else if input.key_pressed(Key::A) {
                if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    buf.select_all();
                }
                return;
            } else if input.key_pressed(Key::C) {
                if let Some(buf) = self.open_buffers.get(self.active_buffer_idx) {
                    if let Some(selected) = buf.selected_text() {
                        ctx.copy_text(selected);
                        self.status_msg = "Copied to clipboard".to_string();
                    }
                }
                return;
            } else if input.key_pressed(Key::X) {
                if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    if let Some(cut_text) = buf.cut_selected_text() {
                        ctx.copy_text(cut_text);
                        self.status_msg = "Cut to clipboard".to_string();
                    }
                }
                return;
            } else if input.key_pressed(Key::Z) {
                if input.modifiers.shift {
                    if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                        buf.redo();
                    }
                } else if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    buf.undo();
                }
                return;
            } else if input.key_pressed(Key::Y) {
                if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    buf.redo();
                }
                return;
            }
        }

        // Editor buffer typing & navigation
        if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
            let shift = input.modifiers.shift;

            if input.key_pressed(Key::ArrowLeft) {
                buf.move_cursor_left(shift);
            } else if input.key_pressed(Key::ArrowRight) {
                buf.move_cursor_right(shift);
            } else if input.key_pressed(Key::ArrowUp) {
                buf.move_cursor_up(shift);
            } else if input.key_pressed(Key::ArrowDown) {
                buf.move_cursor_down(shift);
            } else if input.key_pressed(Key::Backspace) {
                buf.backspace();
            } else if input.key_pressed(Key::Delete) {
                buf.delete_forward();
            } else if input.key_pressed(Key::Enter) {
                buf.insert_newline_with_indent();
            } else if input.key_pressed(Key::Tab) {
                buf.insert_str("    ");
            } else {
                for event in &input.events {
                    match event {
                        egui::Event::Text(text) => {
                            if !text.is_empty() && !input.modifiers.command && !input.modifiers.ctrl {
                                buf.insert_str(text);
                            }
                        }
                        egui::Event::Paste(text) => {
                            if !text.is_empty() {
                                buf.insert_str(text);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

impl eframe::App for VsCodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_keyboard_input(ctx);

        // Top Menubar / Titlebar
        egui::TopBottomPanel::top("top_titlebar")
            .exact_height(36.0)
            .frame(egui::Frame::none().fill(self.theme.sidebar_header).stroke(egui::Stroke::new(1.0, self.theme.border)))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label(RichText::new("Antigravity Code").color(self.theme.accent).strong().size(13.0));

                    // Menus
                    ui.menu_button("File", |ui| {
                        if ui.button("New File (Ctrl+N)").clicked() {
                            self.new_untitled_file();
                            ui.close_menu();
                        }
                        if ui.button("Open File... (Ctrl+O)").clicked() {
                            self.execute_command(ctx, "file.open_file");
                            ui.close_menu();
                        }
                        if ui.button("Open Folder...").clicked() {
                            self.execute_command(ctx, "file.open_folder");
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Save (Ctrl+S)").clicked() {
                            self.execute_command(ctx, "file.save");
                            ui.close_menu();
                        }
                        if ui.button("Close Editor (Ctrl+W)").clicked() {
                            self.execute_command(ctx, "file.close_tab");
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Undo (Ctrl+Z)").clicked() {
                            self.execute_command(ctx, "edit.undo");
                            ui.close_menu();
                        }
                        if ui.button("Redo (Ctrl+Y)").clicked() {
                            self.execute_command(ctx, "edit.redo");
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Find (Ctrl+F)").clicked() {
                            self.find_open = true;
                            ui.close_menu();
                        }
                        if ui.button("Select All (Ctrl+A)").clicked() {
                            self.execute_command(ctx, "edit.select_all");
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Selection", |ui| {
                        if ui.button("Select All (Ctrl+A)").clicked() {
                            self.execute_command(ctx, "edit.select_all");
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("View", |ui| {
                        if ui.button("Command Palette... (Ctrl+Shift+P)").clicked() {
                            self.palette_mode = PaletteMode::Command;
                            self.palette_query = "> ".to_string();
                            self.palette_open = true;
                            ui.close_menu();
                        }
                        if ui.button("Toggle Sidebar (Ctrl+B)").clicked() {
                            self.sidebar_open = !self.sidebar_open;
                            ui.close_menu();
                        }
                        if ui.button("Toggle Terminal (Ctrl+`)").clicked() {
                            self.terminal_open = !self.terminal_open;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Explorer").clicked() {
                            self.active_tab = ActivityBarTab::Explorer;
                            self.sidebar_open = true;
                            ui.close_menu();
                        }
                        if ui.button("Search").clicked() {
                            self.active_tab = ActivityBarTab::Search;
                            self.sidebar_open = true;
                            ui.close_menu();
                        }
                        if ui.button("Source Control").clicked() {
                            self.active_tab = ActivityBarTab::SourceControl;
                            self.sidebar_open = true;
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Terminal", |ui| {
                        if ui.button("Clear Terminal").clicked() {
                            self.execute_command(ctx, "terminal.clear");
                            self.terminal_open = true;
                            ui.close_menu();
                        }
                        if ui.button("Run: cargo check").clicked() {
                            self.execute_command(ctx, "terminal.cargo_check");
                            ui.close_menu();
                        }
                        if ui.button("Run: cargo test").clicked() {
                            self.execute_command(ctx, "terminal.cargo_test");
                            ui.close_menu();
                        }
                        if ui.button("Run: cargo run").clicked() {
                            self.execute_command(ctx, "terminal.cargo_run");
                            ui.close_menu();
                        }
                    });

                    // Search / Command center bar
                    ui.add_space(20.0);
                    let folder_name = self.explorer.root_path.as_ref()
                        .and_then(|p| p.file_name())
                        .and_then(|n| n.to_str())
                        .unwrap_or("Search");
                    let search_bar_text = format!("Search {} (Ctrl+P)", folder_name);
                    if ui.button(RichText::new(search_bar_text).size(11.0).color(self.theme.text_muted)).clicked() {
                        self.palette_mode = PaletteMode::QuickOpen;
                        self.palette_query.clear();
                        if self.cached_files.is_empty() {
                            self.refresh_cached_files();
                        }
                        self.palette_open = true;
                    }

                    // Right header controls
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        if ui.button(if self.terminal_open { "Terminal" } else { "Terminal" })
                            .on_hover_text("Toggle Terminal (Ctrl+`)")
                            .clicked()
                        {
                            self.terminal_open = !self.terminal_open;
                        }
                        if ui.button("Sidebar")
                            .on_hover_text("Toggle Sidebar (Ctrl+B)")
                            .clicked()
                        {
                            self.sidebar_open = !self.sidebar_open;
                        }
                    });
                });
            });

        // Bottom Status Bar
        egui::TopBottomPanel::bottom("status_bar")
            .exact_height(24.0)
            .frame(egui::Frame::none().fill(self.theme.status_bar).stroke(egui::Stroke::new(1.0, self.theme.border)))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(8.0);
                    let branch_str = format!("git: {}", self.git_state.branch);
                    ui.label(RichText::new(branch_str).color(self.theme.accent).size(11.0).strong());
                    ui.add_space(15.0);
                    ui.label(RichText::new(&self.status_msg).color(self.theme.text_muted).size(11.0));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        ui.label(RichText::new("UTF-8").color(self.theme.text_muted).size(11.0));
                        ui.add_space(10.0);
                        ui.label(RichText::new(self.theme.name).color(self.theme.text_muted).size(11.0));
                        ui.add_space(10.0);
                        if let Some(buf) = self.open_buffers.get(self.active_buffer_idx) {
                            let ext = buf.get_extension().to_uppercase();
                            ui.label(RichText::new(ext).color(self.theme.accent).size(11.0));
                            ui.add_space(10.0);
                            let pos_text = format!("Ln {}, Col {}", buf.cursor_line() + 1, buf.cursor_col() + 1);
                            ui.label(RichText::new(pos_text).color(self.theme.text_primary).size(11.0));
                        }
                    });
                });
            });

        // Left Activity Bar
        egui::SidePanel::left("activity_bar")
            .exact_width(48.0)
            .resizable(false)
            .frame(egui::Frame::none().fill(self.theme.activity_bar).stroke(egui::Stroke::new(1.0, self.theme.border)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(4.0);
                    let tabs = [
                        (ActivityBarTab::Explorer, "explorer", "Explorer (Ctrl+Shift+E)"),
                        (ActivityBarTab::Search, "search", "Code Search (Ctrl+Shift+F)"),
                        (ActivityBarTab::SourceControl, "git", "Source Control (Ctrl+Shift+G)"),
                        (ActivityBarTab::RunAndDebug, "debug", "Run and Debug (Ctrl+Shift+D)"),
                        (ActivityBarTab::Remote, "remote", "Remote Explorer"),
                        (ActivityBarTab::Extensions, "extensions", "Extensions (Ctrl+Shift+X)"),
                        (ActivityBarTab::Packages, "package", "Package Manager"),
                    ];
                    for (tab, icon_id, tooltip) in tabs {
                        let is_active = self.active_tab == tab && self.sidebar_open;
                        let btn = MaterialIcon::activity_bar_icon(ui, icon_id, is_active, tooltip);
                        if btn {
                            if self.active_tab == tab && self.sidebar_open {
                                self.sidebar_open = false;
                            } else {
                                self.active_tab = tab;
                                self.sidebar_open = true;
                                if tab == ActivityBarTab::SourceControl {
                                    self.git_state.refresh(self.explorer.root_path.as_deref());
                                }
                            }
                        }
                        ui.add_space(2.0);
                    }

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        let is_settings = self.active_tab == ActivityBarTab::Settings && self.sidebar_open;
                        if MaterialIcon::activity_bar_icon(ui, "settings", is_settings, "Settings") {
                            if self.active_tab == ActivityBarTab::Settings && self.sidebar_open {
                                self.sidebar_open = false;
                            } else {
                                self.active_tab = ActivityBarTab::Settings;
                                self.sidebar_open = true;
                            }
                        }
                    });
                });
            });

        // Primary Sidebar
        if self.sidebar_open {
            egui::SidePanel::left("primary_sidebar")
                .default_width(240.0)
                .width_range(160.0..=500.0)
                .frame(egui::Frame::none().fill(self.theme.sidebar_bg).stroke(egui::Stroke::new(1.0, self.theme.border)))
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.add_space(12.0);
                        let is_search = self.active_tab == ActivityBarTab::Search;
                        let title = match self.active_tab {
                            ActivityBarTab::Explorer => "EXPLORER",
                            ActivityBarTab::Search => "Code Search",
                            ActivityBarTab::SourceControl => "SOURCE CONTROL",
                            ActivityBarTab::RunAndDebug => "RUN AND DEBUG",
                            ActivityBarTab::Remote => "REMOTE EXPLORER",
                            ActivityBarTab::Extensions => "EXTENSIONS",
                            ActivityBarTab::Packages => "PACKAGES",
                            ActivityBarTab::Settings => "SETTINGS",
                        };

                        if is_search {
                            ui.label(RichText::new(title).size(12.5).color(Color32::WHITE).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(6.0);
                                if MaterialIcon::search_header_button(ui, "open_editor", "Open in Editor") {}
                                if MaterialIcon::search_header_button(ui, "collapse_all", "Collapse All") {}
                                if MaterialIcon::search_header_button(ui, "new_search_editor", "New Search Editor") {}
                                if MaterialIcon::search_header_button(ui, "clear", "Clear Search Results") {
                                    self.search_state.query.clear();
                                    self.search_state.hits.clear();
                                }
                                if MaterialIcon::search_header_button(ui, "refresh", "Refresh (Alt+R)") {
                                    if let Some(ref r) = self.explorer.root_path {
                                        self.search_state.execute_search(r);
                                    }
                                }
                            });
                        } else {
                            ui.label(RichText::new(title).size(11.0).color(self.theme.text_muted).strong());
                            if self.active_tab == ActivityBarTab::Explorer {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(8.0);
                                    if ui.small_button("R").on_hover_text("Refresh Explorer").clicked() {
                                        if let Some(ref r) = self.explorer.root_path.clone() {
                                            self.explorer.load_directory(r);
                                            self.refresh_cached_files();
                                        }
                                    }
                                    if ui.small_button("+F").on_hover_text("New Folder").clicked() {
                                        self.new_item_is_dir = true;
                                        self.new_item_dialog_open = true;
                                        self.new_item_name.clear();
                                    }
                                    if ui.small_button("+").on_hover_text("New File").clicked() {
                                        self.new_item_is_dir = false;
                                        self.new_item_dialog_open = true;
                                        self.new_item_name.clear();
                                    }
                                });
                            }
                        }
                    });
                    ui.separator();

                    match self.active_tab {
                        ActivityBarTab::Explorer => {
                            if self.new_item_dialog_open {
                                ui.horizontal(|ui| {
                                    ui.add_space(10.0);
                                    let hint = if self.new_item_is_dir { "Folder name..." } else { "File name..." };
                                    let edit = ui.add(egui::TextEdit::singleline(&mut self.new_item_name).hint_text(hint));
                                    if edit.lost_focus() && ctx.input(|i| i.key_pressed(Key::Enter)) {
                                        if !self.new_item_name.trim().is_empty() {
                                            if let Some(ref root) = self.explorer.root_path {
                                                if self.new_item_is_dir {
                                                    let _ = self.explorer.create_dir(root, &self.new_item_name);
                                                } else {
                                                    if let Ok(path) = self.explorer.create_file(root, &self.new_item_name) {
                                                        self.open_file(&path);
                                                        self.refresh_cached_files();
                                                    }
                                                }
                                            }
                                        }
                                        self.new_item_dialog_open = false;
                                    }
                                    if ui.small_button("Cancel").clicked() {
                                        self.new_item_dialog_open = false;
                                    }
                                });
                                ui.separator();
                            }

                            // Collapsible Directory Tree with Material Theme Icons
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                let items = self.explorer.get_visible_tree();
                                let mut to_open = None;
                                let mut to_toggle = None;

                                for item in items {
                                    ui.horizontal(|ui| {
                                        ui.add_space((item.depth as f32 * 12.0) + 8.0);
                                        let arrow = if item.is_dir {
                                            if item.is_expanded { "v " } else { "> " }
                                        } else {
                                            "  "
                                        };
                                        ui.label(RichText::new(arrow).monospace().color(self.theme.text_muted).size(11.0));

                                        MaterialIcon::render_file_icon(ui, &item.name, item.is_dir, item.is_expanded);
                                        ui.add_space(2.0);

                                        let response = ui.selectable_label(false, RichText::new(&item.name).color(self.theme.text_primary).size(12.5));
                                        if response.clicked() {
                                            if item.is_dir {
                                                to_toggle = Some(item.path);
                                            } else {
                                                to_open = Some(item.path);
                                            }
                                        }
                                    });
                                }

                                if let Some(p) = to_toggle {
                                    self.explorer.toggle_expand(&p);
                                }
                                if let Some(p) = to_open {
                                    self.open_file(&p);
                                }
                            });
                        }

                        ActivityBarTab::Search => {
                            ui.add_space(4.0);
                            // Row 1: Left chevron / spacing + Search input with Aa, ab, .*
                            ui.horizontal(|ui| {
                                ui.add_space(4.0);
                                if !self.search_show_replace {
                                    if MaterialIcon::chevron_button(ui, false, "Toggle Replace") {
                                        self.search_show_replace = true;
                                    }
                                } else {
                                    ui.add_space(14.0);
                                }

                                let total_w = ui.available_width() - 8.0;
                                egui::Frame::none()
                                    .fill(Color32::from_rgb(32, 32, 35))
                                    .stroke(egui::Stroke::new(1.0, Color32::from_rgb(52, 52, 58)))
                                    .rounding(2.0)
                                    .inner_margin(egui::Margin::symmetric(6.0, 3.0))
                                    .show(ui, |ui| {
                                        ui.set_width(total_w);
                                        ui.horizontal(|ui| {
                                            let text_w = ui.available_width() - 74.0;
                                            let search_edit = ui.add(
                                                egui::TextEdit::singleline(&mut self.search_state.query)
                                                    .hint_text("Search")
                                                    .desired_width(text_w.max(60.0))
                                                    .frame(false)
                                            );

                                            if (search_edit.lost_focus() && ctx.input(|i| i.key_pressed(Key::Enter)))
                                                || search_edit.changed()
                                            {
                                                if let Some(ref r) = self.explorer.root_path {
                                                    self.search_state.execute_search(r);
                                                }
                                            }

                                            MaterialIcon::search_toggle(ui, &mut self.search_state.case_sensitive, "Aa", "Match Case (Alt+C)");
                                            MaterialIcon::search_toggle(ui, &mut self.search_state.match_whole_word, "ab", "Match Whole Word (Alt+W)");
                                            MaterialIcon::search_toggle(ui, &mut self.search_state.use_regex, ".*", "Use Regular Expression (Alt+R)");
                                        });
                                    });
                            });

                            // Row 2: Replace input with AB, Replace All
                            if self.search_show_replace {
                                ui.add_space(3.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(4.0);
                                    if MaterialIcon::chevron_button(ui, true, "Toggle Replace") {
                                        self.search_show_replace = false;
                                    }

                                    let total_w = ui.available_width() - 8.0;
                                    egui::Frame::none()
                                        .fill(Color32::from_rgb(32, 32, 35))
                                        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(52, 52, 58)))
                                        .rounding(2.0)
                                        .inner_margin(egui::Margin::symmetric(6.0, 3.0))
                                        .show(ui, |ui| {
                                            ui.set_width(total_w);
                                            ui.horizontal(|ui| {
                                                let text_w = ui.available_width() - 50.0;
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut self.search_state.replace_query)
                                                        .hint_text("Replace")
                                                        .desired_width(text_w.max(60.0))
                                                        .frame(false)
                                                );

                                                MaterialIcon::search_toggle(ui, &mut self.search_state.preserve_case, "AB", "Preserve Case (Alt+P)");

                                                if MaterialIcon::replace_all_button(ui, "Replace All (Ctrl+Alt+Enter)") {
                                                    if let Some(ref r) = self.explorer.root_path {
                                                        self.search_state.execute_replace_all(r);
                                                    }
                                                }
                                            });
                                        });
                                });

                                // Row 3: Details toggle button `...`
                                ui.horizontal(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.add_space(10.0);
                                        if MaterialIcon::details_button(ui, self.search_state.show_details, "Toggle Search Details") {
                                            self.search_state.show_details = !self.search_state.show_details;
                                        }
                                    });
                                });

                                if self.search_state.show_details {
                                    ui.add_space(2.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(20.0);
                                        ui.label(RichText::new("files to include").size(10.5).color(self.theme.text_muted));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.add_space(20.0);
                                        let w = ui.available_width() - 8.0;
                                        ui.add(egui::TextEdit::singleline(&mut self.search_state.files_to_include).desired_width(w));
                                    });

                                    ui.add_space(2.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(20.0);
                                        ui.label(RichText::new("files to exclude").size(10.5).color(self.theme.text_muted));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.add_space(20.0);
                                        let w = ui.available_width() - 8.0;
                                        ui.add(egui::TextEdit::singleline(&mut self.search_state.files_to_exclude).desired_width(w));
                                    });
                                }
                            }

                            if !self.search_state.replace_message.is_empty() {
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(12.0);
                                    ui.label(RichText::new(&self.search_state.replace_message).size(11.0).color(self.theme.accent));
                                });
                            }

                            ui.separator();

                            // Results header
                            let hit_count = self.search_state.hits.len();
                            if !self.search_state.query.trim().is_empty() {
                                ui.horizontal(|ui| {
                                    ui.add_space(10.0);
                                    let msg = if hit_count == 0 {
                                        "No results found.".to_string()
                                    } else {
                                        format!("{} results found", hit_count)
                                    };
                                    ui.label(RichText::new(msg).size(11.0).color(self.theme.text_muted));
                                });
                            }

                            // Results tree: grouped by file
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                let mut jump_to = None;
                                let mut grouped: std::collections::BTreeMap<String, Vec<(usize, String, PathBuf)>> = std::collections::BTreeMap::new();
                                for hit in &self.search_state.hits {
                                    grouped.entry(hit.relative_path.clone())
                                        .or_default()
                                        .push((hit.line_num, hit.line_content.clone(), hit.path.clone()));
                                }

                                for (rel_path, matches) in grouped {
                                    ui.horizontal(|ui| {
                                        ui.add_space(8.0);
                                        ui.label(RichText::new("v").monospace().size(10.5).color(self.theme.text_muted));
                                        MaterialIcon::render_file_icon(ui, &rel_path, false, false);
                                        ui.add_space(2.0);
                                        let header = format!("{} ({})", rel_path, matches.len());
                                        ui.label(RichText::new(header).size(12.0).color(Color32::WHITE).strong());
                                    });

                                    for (line_num, content, path) in matches {
                                        ui.horizontal(|ui| {
                                            ui.add_space(26.0);
                                            let label = format!("{}: {}", line_num, content);
                                            if ui.selectable_label(false, RichText::new(label).size(11.5).color(self.theme.text_primary)).clicked() {
                                                jump_to = Some((path, line_num));
                                            }
                                        });
                                    }
                                }

                                if let Some((path, line)) = jump_to {
                                    self.open_file(&path);
                                    if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                                        buf.go_to_line(line);
                                    }
                                }
                            });
                        }

                        ActivityBarTab::RunAndDebug => {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("RUN AND DEBUG").strong().size(11.0).color(self.theme.text_muted));
                                });
                                ui.add_space(6.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    if ui.button("▷ Debug vscode-native (F5)").clicked() {
                                        self.send_to_active_terminal("cargo build\n");
                                        self.terminal_open = true;
                                    }
                                });
                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    if ui.button("▷ Run cargo test").clicked() {
                                        self.send_to_active_terminal("cargo test\n");
                                        self.terminal_open = true;
                                    }
                                });
                                ui.separator();
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("VARIABLES").strong().size(11.0).color(self.theme.text_muted));
                                });
                                ui.horizontal(|ui| {
                                    ui.add_space(16.0);
                                    ui.label(RichText::new("No active debug session").size(11.0).color(self.theme.text_muted));
                                });
                                ui.separator();
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("WATCH").strong().size(11.0).color(self.theme.text_muted));
                                });
                            });
                        }

                        ActivityBarTab::Remote => {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("REMOTE EXPLORER").strong().size(11.0).color(self.theme.text_muted));
                                });
                                ui.add_space(6.0);
                                let remotes = [
                                    ("Local Environment", "Active"),
                                    ("SSH: Development Server", "Available"),
                                    ("Dev Containers", "Available"),
                                    ("WSL: Ubuntu", "Available"),
                                ];
                                for (target, status) in remotes {
                                    ui.horizontal(|ui| {
                                        ui.add_space(8.0);
                                        ui.label(RichText::new(target).color(self.theme.text_primary));
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.add_space(8.0);
                                            ui.label(RichText::new(status).color(self.theme.accent).size(10.5));
                                        });
                                    });
                                    ui.add_space(2.0);
                                }
                            });
                        }

                        ActivityBarTab::Packages => {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    ui.label(RichText::new("PACKAGES & CRATES").strong().size(11.0).color(self.theme.text_muted));
                                });
                                ui.add_space(6.0);
                                let packages = [
                                    ("vscode-native", "0.1.0", "Workspace Root"),
                                    ("eframe", "0.29", "GUI Framework"),
                                    ("egui", "0.29", "Immediate UI"),
                                    ("ropey", "1.6", "Text Buffer Engine"),
                                    ("syntect", "5.2", "Syntax Highlighting"),
                                    ("portable-pty", "0.8", "Pseudo-Terminal"),
                                    ("walkdir", "2.5", "Directory Walker"),
                                    ("rfd", "0.15", "Native File Dialogs"),
                                ];
                                for (name, ver, desc) in packages {
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(name).strong().color(self.theme.text_primary));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(RichText::new(ver).color(self.theme.accent).monospace().size(10.5));
                                            });
                                        });
                                        ui.label(RichText::new(desc).size(11.0).color(self.theme.text_muted));
                                    });
                                    ui.add_space(2.0);
                                }
                            });
                        }


                        ActivityBarTab::SourceControl => {
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.label(RichText::new(format!("Branch: {}", self.git_state.branch)).strong().color(self.theme.accent));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(8.0);
                                    if ui.small_button("Refresh").clicked() {
                                        self.git_state.refresh(self.explorer.root_path.as_deref());
                                    }
                                });
                            });

                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.add(egui::TextEdit::singleline(&mut self.git_state.commit_message).hint_text("Commit message"));
                            });

                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                if ui.button("Commit Changes").clicked() {
                                    self.git_state.commit(self.explorer.root_path.as_deref());
                                }
                            });

                            if !self.git_state.status_feedback.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    ui.label(RichText::new(&self.git_state.status_feedback).size(11.0).color(self.theme.accent));
                                });
                            }

                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.label(RichText::new(format!("Changes ({})", self.git_state.changes.len())).size(11.0).strong().color(self.theme.text_muted));
                            });

                            egui::ScrollArea::vertical().show(ui, |ui| {
                                let mut open_target = None;
                                for change in &self.git_state.changes {
                                    ui.horizontal(|ui| {
                                        ui.add_space(8.0);
                                        let status_color = match change.status.as_str() {
                                            "M" => Color32::from_rgb(226, 192, 141),
                                            "A" | "??" => Color32::from_rgb(115, 201, 145),
                                            "D" => Color32::from_rgb(241, 76, 76),
                                            _ => self.theme.text_muted,
                                        };
                                        ui.label(RichText::new(&change.status).color(status_color).size(11.0).strong());
                                        MaterialIcon::render_file_icon(ui, &change.display_name, false, false);
                                        if ui.selectable_label(false, RichText::new(&change.display_name).size(12.0).color(self.theme.text_primary)).clicked() {
                                            open_target = Some(change.path.clone());
                                        }
                                    });
                                }
                                if let Some(target) = open_target {
                                    self.open_file(&target);
                                }
                            });
                        }

                        ActivityBarTab::Extensions => {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.add_space(8.0);
                                let extensions = [
                                    ("Material Theme Icons", "Installed & Enabled by Default", "Active"),
                                    ("rust-analyzer", "Rust language support and analyzer", "Active"),
                                    ("CodeLLDB", "Native debugger for Rust, C++, and Swift", "Active"),
                                    ("GitLens", "Supercharge Git within Antigravity Code", "Active"),
                                    ("Even Better TOML", "Fully-featured TOML support", "Active"),
                                    ("Error Lens", "Highlight errors and warnings inline", "Active"),
                                ];
                                for (name, desc, status) in extensions {
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(name).strong().color(self.theme.text_primary));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(RichText::new(status).color(self.theme.accent).size(10.5));
                                            });
                                        });
                                        ui.label(RichText::new(desc).size(11.0).color(self.theme.text_muted));
                                    });
                                    ui.add_space(4.0);
                                }
                            });
                        }

                        ActivityBarTab::Settings => {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.add_space(8.0);
                                ui.label(RichText::new("Color Theme").strong().color(self.theme.text_primary));
                                let themes = [
                                    (ThemePreset::Antigravity, "Antigravity Dark (Default)"),
                                    (ThemePreset::DarkPlus, "Dark+ (VS Code Default)"),
                                    (ThemePreset::Monokai, "Monokai"),
                                    (ThemePreset::SolarizedDark, "Solarized Dark"),
                                    (ThemePreset::GitHubDark, "GitHub Dark"),
                                    (ThemePreset::LightPlus, "Light+"),
                                ];
                                for (preset, name) in themes {
                                    let selected = self.theme.preset == preset;
                                    if ui.selectable_label(selected, name).clicked() {
                                        self.set_theme_preset(ctx, preset);
                                    }
                                }

                                ui.separator();
                                ui.label(RichText::new("Editor Font Size").strong().color(self.theme.text_primary));
                                ui.add(egui::Slider::new(&mut self.editor_font_size, 10.0..=22.0).text("pt"));

                                ui.separator();
                                ui.label(RichText::new("Icon Theme").strong().color(self.theme.text_primary));
                                ui.label(RichText::new("Material Theme Icons (Installed & Active)").color(self.theme.accent).size(11.5));

                                ui.separator();
                                ui.label(RichText::new("Keyboard Shortcuts").strong().color(self.theme.text_primary));
                                let shortcuts = [
                                    ("Ctrl+Shift+P", "Command Palette"),
                                    ("Ctrl+P", "Quick Open Files"),
                                    ("Ctrl+F", "Find in File"),
                                    ("Ctrl+G", "Go to Line"),
                                    ("Ctrl+N", "New File"),
                                    ("Ctrl+S", "Save File"),
                                    ("Ctrl+W", "Close Editor"),
                                    ("Ctrl+B", "Toggle Sidebar"),
                                    ("Ctrl+`", "Toggle Terminal"),
                                ];
                                for (key, desc) in shortcuts {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(key).monospace().color(self.theme.accent));
                                        ui.label(RichText::new(desc).size(11.5).color(self.theme.text_muted));
                                    });
                                }
                            });
                        }
                    }
                });
        }

        // Bottom Integrated Terminal Panel
        if self.terminal_open {
            let term_height = if self.terminal_maximized {
                460.0
            } else {
                220.0
            };

            egui::TopBottomPanel::bottom("terminal_panel")
                .default_height(term_height)
                .height_range(140.0..=600.0)
                .resizable(true)
                .frame(egui::Frame::none().fill(self.theme.editor_bg).stroke(egui::Stroke::new(1.0, self.theme.border)))
                .show(ctx, |ui| {
                    // Terminal Header Bar: Problems  Output  Debug Console  Terminal (active)  Ports  |  + v  ...  [□]  ✕
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        let tabs = ["Problems", "Output", "Debug Console", "Terminal", "Ports"];
                        for tab_name in tabs {
                            let is_active = self.terminal_tab == tab_name;
                            let text_color = if is_active { Color32::WHITE } else { self.theme.text_muted };
                            let label = if tab_name == "Problems" {
                                "Problems 0".to_string()
                            } else {
                                tab_name.to_string()
                            };

                            let resp = ui.selectable_label(false, RichText::new(label).size(11.5).color(text_color));
                            if resp.clicked() {
                                self.terminal_tab = tab_name.to_string();
                            }

                            if is_active {
                                let r = resp.rect;
                                let line = [Pos2::new(r.min.x, r.max.y), Pos2::new(r.max.x, r.max.y)];
                                ui.painter().line_segment(line, egui::Stroke::new(1.5, Color32::from_rgb(0, 122, 204)));
                            }
                            ui.add_space(6.0);
                        }

                        // Right header actions: + v, ..., [□], ✕
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(6.0);
                            if MaterialIcon::terminal_action_button(ui, "close", "Close Panel") {
                                self.terminal_open = false;
                            }
                            if MaterialIcon::terminal_action_button(ui, "maximize", "Toggle Panel Size") {
                                self.terminal_maximized = !self.terminal_maximized;
                            }
                            if MaterialIcon::terminal_action_button(ui, "more", "More Actions") {}
                            if MaterialIcon::terminal_action_button(ui, "add", "New Terminal") {
                                self.terminal_mgr.new_session(None);
                            }
                        });
                    });

                    ui.separator();

                    // Terminal Panel Body: Left output + Right sessions list
                    let total_w = ui.available_width();
                    let sidebar_w = 135.0;
                    let output_w = (total_w - sidebar_w).max(100.0);
                    let body_h = ui.available_height();

                    ui.horizontal(|ui| {
                        // Left: Terminal Output
                        ui.allocate_ui_with_layout(
                            Vec2::new(output_w, body_h),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                egui::ScrollArea::vertical()
                                    .stick_to_bottom(true)
                                    .max_height(body_h - 28.0)
                                    .show(ui, |ui| {
                                        ui.add_space(4.0);
                                        if let Some(session) = self.terminal_mgr.active_session() {
                                            let text = session.terminal.output.lock().unwrap_or_else(|e| e.into_inner()).clone();
                                            ui.label(RichText::new(text).font(FontId::monospace(12.0)).color(self.theme.text_primary));
                                        }
                                    });

                                // In-place command input with circle glyph 'o' like screenshot
                                ui.horizontal(|ui| {
                                    ui.add_space(4.0);
                                    ui.label(RichText::new("o").monospace().size(11.0).color(Color32::from_rgb(130, 130, 140)));
                                    ui.add_space(4.0);
                                    let edit = ui.add(
                                        egui::TextEdit::singleline(&mut self.terminal_input)
                                            .hint_text("Type shell command...")
                                            .desired_width(ui.available_width() - 10.0)
                                            .frame(false)
                                            .font(FontId::monospace(12.0)),
                                    );
                                    if (edit.lost_focus() && ctx.input(|i| i.key_pressed(Key::Enter)))
                                        || (edit.has_focus() && ctx.input(|i| i.key_pressed(Key::Enter)))
                                    {
                                        if !self.terminal_input.trim().is_empty() {
                                            let cmd = format!("{}\n", self.terminal_input);
                                            self.send_to_active_terminal(&cmd);
                                            self.terminal_input.clear();
                                        }
                                    }
                                });
                            },
                        );

                        // Vertical divider line
                        let (sep_rect, _) = ui.allocate_exact_size(Vec2::new(1.0, body_h), egui::Sense::hover());
                        ui.painter().rect_filled(sep_rect, 0.0, Color32::from_rgb(39, 39, 42));

                        // Right: Terminal Sessions Sidebar matching screenshot
                        ui.allocate_ui_with_layout(
                            Vec2::new(sidebar_w - 2.0, body_h),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                egui::Frame::none()
                                    .fill(self.theme.sidebar_bg)
                                    .show(ui, |ui| {
                                        ui.add_space(4.0);
                                        let mut session_to_close = None;

                                        for (i, session) in self.terminal_mgr.sessions.iter().enumerate() {
                                            let is_active = i == self.terminal_mgr.active_idx;
                                            let bg = if is_active {
                                                Color32::from_rgb(43, 45, 56)
                                            } else {
                                                Color32::TRANSPARENT
                                            };

                                            let frame = egui::Frame::none()
                                                .fill(bg)
                                                .inner_margin(egui::Margin::symmetric(6.0, 4.0));

                                            frame.show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    if is_active {
                                                        let (bar_rect, _) = ui.allocate_exact_size(Vec2::new(2.0, 14.0), egui::Sense::hover());
                                                        ui.painter().rect_filled(bar_rect, 0.5, Color32::from_rgb(0, 122, 204));
                                                        ui.add_space(4.0);
                                                    } else {
                                                        ui.add_space(6.0);
                                                    }

                                                    MaterialIcon::terminal_session_icon(ui, is_active);
                                                    ui.add_space(4.0);

                                                    let text_color = if is_active { Color32::WHITE } else { self.theme.text_muted };
                                                    let label_resp = ui.selectable_label(false, RichText::new(&session.name).size(12.0).color(text_color));
                                                    if label_resp.clicked() {
                                                        self.terminal_mgr.active_idx = i;
                                                    }

                                                    if self.terminal_mgr.sessions.len() > 1 {
                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                            if MaterialIcon::close_button(ui, 10.0) {
                                                                session_to_close = Some(i);
                                                            }
                                                        });
                                                    }
                                                });
                                            });
                                            ui.add_space(2.0);
                                        }

                                        if let Some(idx) = session_to_close {
                                            self.terminal_mgr.close_session(idx);
                                        }
                                    });
                            },
                        );
                    });
                });
        }

        // Central Area: Tab Bar + Editor Buffer + Find Bar + Command Palette
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(self.theme.editor_bg))
            .show(ctx, |ui| {
                // Editor Tabs Header
                ui.horizontal(|ui| {
                    let mut tab_to_close = None;

                    for (i, buf) in self.open_buffers.iter().enumerate() {
                        let is_active = i == self.active_buffer_idx;
                        let tab_bg = if is_active { self.theme.tab_active } else { self.theme.tab_inactive };
                        let dirty_mark = if buf.is_dirty { " *" } else { "" };

                        egui::Frame::none()
                            .fill(tab_bg)
                            .stroke(if is_active { egui::Stroke::new(1.0, self.theme.accent) } else { egui::Stroke::new(1.0, self.theme.border) })
                            .inner_margin(egui::Margin::symmetric(10.0, 5.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    MaterialIcon::render_file_icon(ui, &buf.file_name, false, false);
                                    ui.add_space(4.0);

                                    let label_text = format!("{}{}", buf.file_name, dirty_mark);
                                    let label_color = if is_active { Color32::WHITE } else { self.theme.text_muted };
                                    if ui.selectable_label(is_active, RichText::new(label_text).color(label_color)).clicked() {
                                        self.active_buffer_idx = i;
                                    }

                                    // Vector close button
                                    if MaterialIcon::close_button(ui, 12.0) {
                                        tab_to_close = Some(i);
                                    }
                                });
                            });
                    }

                    // Add new tab button
                    if ui.button("+").on_hover_text("New File (Ctrl+N)").clicked() {
                        self.new_untitled_file();
                    }

                    if let Some(idx) = tab_to_close {
                        self.close_tab(idx);
                    }
                });

                ui.separator();

                // Find & Replace bar widget
                if self.find_open {
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.label(RichText::new("Find:").size(11.0).color(self.theme.text_muted));
                        let find_edit = ui.add(egui::TextEdit::singleline(&mut self.find_query).hint_text("Search in editor"));
                        if find_edit.lost_focus() && ctx.input(|i| i.key_pressed(Key::Enter)) {
                            if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                                buf.find_next(&self.find_query, self.find_case_sensitive);
                            }
                        }

                        if ui.small_button("Next").clicked() {
                            if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                                buf.find_next(&self.find_query, self.find_case_sensitive);
                            }
                        }
                        if ui.small_button("Prev").clicked() {
                            if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                                buf.find_prev(&self.find_query, self.find_case_sensitive);
                            }
                        }

                        ui.checkbox(&mut self.find_case_sensitive, "Aa");

                        if let Some(buf) = self.open_buffers.get(self.active_buffer_idx) {
                            let count = buf.count_matches(&self.find_query, self.find_case_sensitive);
                            ui.label(RichText::new(format!("{} matches", count)).size(11.0).color(self.theme.text_muted));
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if MaterialIcon::close_button(ui, 14.0) {
                                self.find_open = false;
                            }
                        });
                    });
                    ui.separator();
                }

                // If no buffers are open: Empty State
                if self.open_buffers.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(80.0);
                        ui.label(RichText::new("Antigravity Code").size(20.0).strong().color(self.theme.text_primary));
                        ui.add_space(10.0);
                        ui.label(RichText::new("Command Palette: Ctrl+Shift+P").color(self.theme.text_muted));
                        ui.label(RichText::new("Quick Open: Ctrl+P").color(self.theme.text_muted));
                        ui.label(RichText::new("Find in Files: Ctrl+Shift+F").color(self.theme.text_muted));
                        ui.label(RichText::new("Toggle Terminal: Ctrl+`").color(self.theme.text_muted));
                        ui.add_space(20.0);
                        if ui.button("Create New File").clicked() {
                            self.new_untitled_file();
                        }
                    });
                    return;
                }

                // High-performance virtualized editor buffer area with Minimap
                if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    let line_count = buf.line_count();
                    let ext = buf.get_extension().to_string();
                    let font_id = FontId::monospace(self.editor_font_size);
                    let line_height = self.editor_font_size + 5.0;
                    let cur_line = buf.cursor_line();
                    let cur_col = buf.cursor_col();

                    let mut clicked_line = None;
                    let minimap_width = 72.0;
                    let editor_w = (ui.available_width() - minimap_width).max(180.0);
                    let available_h = ui.available_height();
                    let mut visible_rows = 0..line_count.min(30);

                    ui.horizontal(|ui| {
                        // Left: Code Editor and Gutter
                        ui.allocate_ui_with_layout(
                            Vec2::new(editor_w, available_h),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                egui::ScrollArea::both()
                                    .auto_shrink([false, false])
                                    .show_rows(ui, line_height, line_count, |ui, row_range| {
                                        visible_rows = row_range.clone();
                                        ui.horizontal_top(|ui| {
                                            // Gutter: Line Numbers right-aligned
                                            ui.vertical(|ui| {
                                                ui.set_width(42.0);
                                                for idx in row_range.clone() {
                                                    let num_str = format!("{:>4}", idx + 1);
                                                    let col = if idx == cur_line { Color32::WHITE } else { self.theme.line_numbers };
                                                    ui.label(RichText::new(num_str).font(font_id.clone()).color(col));
                                                }
                                            });

                                            ui.add_space(8.0); // Clean spacing instead of harsh separator line

                                            // Highlighted Code Lines for visible rows ONLY
                                            ui.vertical(|ui| {
                                                for idx in row_range {
                                                    let raw_line = buf.line_str(idx).unwrap_or_default();
                                                    let tokens = self.syntax_highlighter.highlight_line(&raw_line, &ext);

                                                    let line_response = ui.horizontal(|ui| {
                                                        ui.spacing_mut().item_spacing.x = 0.0;
                                                        let mut col_offset = 0;

                                                        for (color, token_text) in tokens {
                                                            let clean_text = token_text.trim_end_matches(['\r', '\n']);
                                                            if clean_text.is_empty() {
                                                                continue;
                                                            }

                                                            let token_len = clean_text.chars().count();
                                                            if idx == cur_line && cur_col >= col_offset && cur_col < col_offset + token_len {
                                                                let split_pos = cur_col - col_offset;
                                                                let before: String = clean_text.chars().take(split_pos).collect();
                                                                let at_cursor: String = clean_text.chars().skip(split_pos).take(1).collect();
                                                                let after: String = clean_text.chars().skip(split_pos + 1).collect();

                                                                if !before.is_empty() {
                                                                    ui.label(RichText::new(before).font(font_id.clone()).color(color));
                                                                }
                                                                ui.label(
                                                                    RichText::new(if at_cursor.is_empty() { " " } else { &at_cursor })
                                                                        .font(font_id.clone())
                                                                        .background_color(self.theme.accent)
                                                                        .color(Color32::BLACK),
                                                                );
                                                                if !after.is_empty() {
                                                                    ui.label(RichText::new(after).font(font_id.clone()).color(color));
                                                                }
                                                            } else {
                                                                ui.label(RichText::new(clean_text).font(font_id.clone()).color(color));
                                                            }
                                                            col_offset += token_len;
                                                        }

                                                        if idx == cur_line && cur_col >= col_offset {
                                                            ui.label(
                                                                RichText::new(" ")
                                                                    .font(font_id.clone())
                                                                    .background_color(self.theme.accent),
                                                            );
                                                        }
                                                    }).response;

                                                    if line_response.interact(egui::Sense::click()).clicked() {
                                                        clicked_line = Some(idx);
                                                    }
                                                }
                                            });
                                        });
                                    });
                            },
                        );

                        // Right: VS Code Minimap Column
                        let (map_rect, map_resp) = ui.allocate_exact_size(
                            Vec2::new(minimap_width, available_h),
                            egui::Sense::click_and_drag(),
                        );

                        let painter = ui.painter();
                        painter.rect_filled(map_rect, 0.0, Color32::from_rgb(18, 18, 22));
                        painter.line_segment([map_rect.left_top(), map_rect.left_bottom()], egui::Stroke::new(1.0, Color32::from_rgb(34, 34, 40)));

                        let map_h = map_rect.height();
                        let total_l = line_count.max(1) as f32;
                        let step_y = (map_h / total_l).clamp(1.5, 3.5);

                        for i in 0..line_count.min(300) {
                            if let Some(l) = buf.line_str(i) {
                                let trimmed = l.trim_start();
                                let indent = ((l.len() - trimmed.len()).min(25) as f32) * 1.5;
                                let line_w = (trimmed.len() as f32 * 1.6).clamp(3.0, minimap_width - indent - 8.0);
                                let y = map_rect.min.y + (i as f32 * step_y);
                                if y > map_rect.max.y {
                                    break;
                                }
                                let color = if trimmed.starts_with("//") || trimmed.starts_with('#') {
                                    Color32::from_rgb(85, 95, 105)
                                } else if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") || trimmed.starts_with("pub struct ") {
                                    Color32::from_rgb(225, 180, 90)
                                } else {
                                    Color32::from_rgb(130, 135, 145)
                                };
                                painter.line_segment(
                                    [Pos2::new(map_rect.min.x + 4.0 + indent, y), Pos2::new(map_rect.min.x + 4.0 + indent + line_w, y)],
                                    egui::Stroke::new(1.2, color),
                                );
                            }
                        }

                        // Viewport indicator slider box
                        let vp_top = map_rect.min.y + (visible_rows.start as f32 * step_y);
                        let vp_h = ((visible_rows.end - visible_rows.start) as f32 * step_y).max(22.0);
                        let vp_rect = Rect::from_min_size(
                            Pos2::new(map_rect.min.x, vp_top.min(map_rect.max.y - vp_h)),
                            Vec2::new(minimap_width, vp_h),
                        );
                        painter.rect_filled(vp_rect, 1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 18));
                        painter.rect_stroke(vp_rect, 1.0, egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 45)));

                        if map_resp.clicked() || map_resp.dragged() {
                            if let Some(pos) = map_resp.interact_pointer_pos() {
                                let rel_y = (pos.y - map_rect.min.y).clamp(0.0, map_h);
                                let target_line = ((rel_y / map_h) * total_l) as usize;
                                clicked_line = Some(target_line.min(line_count.saturating_sub(1)));
                            }
                        }
                    });

                    if let Some(target_line) = clicked_line {
                        buf.set_cursor_from_line_col(target_line, 0);
                    }
                }
            });

        // Floating Command Palette / Quick Open Window
        if self.palette_open {
            egui::Window::new("palette_modal")
                .title_bar(false)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_TOP, egui::Vec2::new(0.0, 45.0))
                .fixed_size(egui::Vec2::new(560.0, 320.0))
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(self.theme.sidebar_bg)
                        .stroke(egui::Stroke::new(1.0, self.theme.accent)),
                )
                .show(ctx, |ui| {
                    ui.add_space(4.0);
                    let placeholder = match self.palette_mode {
                        PaletteMode::Command => "> Type a command...",
                        PaletteMode::QuickOpen => "Type a file name to open...",
                        PaletteMode::GoToLine => ":Go to line number...",
                    };

                    let text_edit = ui.add(
                        egui::TextEdit::singleline(&mut self.palette_query)
                            .hint_text(placeholder)
                            .desired_width(ui.available_width() - 10.0),
                    );
                    text_edit.request_focus();

                    ui.separator();

                    match self.palette_mode {
                        PaletteMode::Command => {
                            let commands = get_all_commands();
                            let filter = self.palette_query.trim_start_matches('>').trim().to_lowercase();
                            let matched: Vec<&CommandItem> = commands
                                .iter()
                                .filter(|c| c.title.to_lowercase().contains(&filter) || c.category.to_lowercase().contains(&filter))
                                .collect();

                            let mut chosen_cmd = None;
                            egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                                for (i, cmd) in matched.iter().enumerate() {
                                    let is_selected = i == self.palette_selected_idx;
                                    let label = format!("{}: {}", cmd.category, cmd.title);
                                    let mut text = RichText::new(label).size(12.5).color(self.theme.text_primary);
                                    if is_selected {
                                        text = text.strong().color(Color32::WHITE);
                                    }

                                    ui.horizontal(|ui| {
                                        if ui.selectable_label(is_selected, text).clicked() {
                                            chosen_cmd = Some(cmd.id);
                                        }
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if !cmd.keybinding.is_empty() {
                                                ui.label(RichText::new(cmd.keybinding).size(11.0).color(self.theme.text_muted));
                                            }
                                        });
                                    });
                                }
                            });

                            if ctx.input(|i| i.key_pressed(Key::Enter)) {
                                if let Some(cmd) = matched.get(self.palette_selected_idx) {
                                    chosen_cmd = Some(cmd.id);
                                }
                            }

                            if let Some(cmd_id) = chosen_cmd {
                                self.execute_command(ctx, cmd_id);
                            }
                        }

                        PaletteMode::QuickOpen => {
                            let filter = self.palette_query.trim().to_lowercase();
                            let matched_files: Vec<&(String, PathBuf)> = self.cached_files
                                .iter()
                                .filter(|(name, _)| filter.is_empty() || name.to_lowercase().contains(&filter))
                                .take(30)
                                .collect();

                            let mut chosen_file = None;
                            egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                                for (i, (name, path)) in matched_files.iter().enumerate() {
                                    let is_selected = i == self.palette_selected_idx;
                                    let text = RichText::new(name).size(12.5).color(if is_selected { Color32::WHITE } else { self.theme.text_primary });

                                    ui.horizontal(|ui| {
                                        MaterialIcon::render_file_icon(ui, name, false, false);
                                        if ui.selectable_label(is_selected, text).clicked() {
                                            chosen_file = Some(path.clone());
                                        }
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.label(RichText::new(path.to_string_lossy()).size(10.5).color(self.theme.text_muted));
                                        });
                                    });
                                }
                            });

                            if ctx.input(|i| i.key_pressed(Key::Enter)) {
                                if let Some((_, path)) = matched_files.get(self.palette_selected_idx) {
                                    chosen_file = Some((*path).clone());
                                }
                            }

                            if let Some(path) = chosen_file {
                                self.open_file(&path);
                                self.palette_open = false;
                            }
                        }

                        PaletteMode::GoToLine => {
                            let line_str = self.palette_query.trim_start_matches(':').trim();
                            if let Ok(line_num) = line_str.parse::<usize>() {
                                ui.label(RichText::new(format!("Press Enter to go to line {}", line_num)).size(12.0).color(self.theme.accent));
                                if ctx.input(|i| i.key_pressed(Key::Enter)) {
                                    if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                                        buf.go_to_line(line_num);
                                    }
                                    self.palette_open = false;
                                }
                            } else {
                                ui.label(RichText::new("Enter a line number to jump to").size(12.0).color(self.theme.text_muted));
                            }
                        }
                    }
                });
        }
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(1100.0, 750.0))
            .with_min_inner_size(Vec2::new(600.0, 400.0))
            .with_title("Visual Studio Code (Native Rust)"),
        ..Default::default()
    };

    eframe::run_native(
        "Visual Studio Code",
        native_options,
        Box::new(|cc| Ok(Box::new(VsCodeApp::new(cc)))),
    )
}
