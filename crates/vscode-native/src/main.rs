mod buffer;
mod explorer;
mod syntax;
mod terminal;
mod theme;

use buffer::EditorBuffer;
use explorer::ExplorerState;
use syntax::SyntaxHighlighter;
use terminal::Terminal;
use theme::*;

use eframe::egui::{self, Color32, FontId, Key, RichText, Vec2};
use std::path::Path;

#[derive(PartialEq, Eq)]
enum ActivityBarTab {
    Explorer,
    Search,
    SourceControl,
    Extensions,
    Settings,
}

struct VsCodeApp {
    active_tab: ActivityBarTab,
    explorer: ExplorerState,
    open_buffers: Vec<EditorBuffer>,
    active_buffer_idx: usize,
    syntax_highlighter: SyntaxHighlighter,
    terminal: Option<Terminal>,
    terminal_open: bool,
    status_msg: String,
}

impl VsCodeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::setup_vscode_visuals(&cc.egui_ctx);

        let current_dir = std::env::current_dir().ok();
        let explorer = ExplorerState::new(current_dir);
        let initial_buffer = EditorBuffer::empty();
        let terminal = Terminal::new().ok();

        Self {
            active_tab: ActivityBarTab::Explorer,
            explorer,
            open_buffers: vec![initial_buffer],
            active_buffer_idx: 0,
            syntax_highlighter: SyntaxHighlighter::new(),
            terminal,
            terminal_open: true,
            status_msg: "Ready".to_string(),
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

    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| i.clone());

        // Keyboard shortcuts
        if input.modifiers.command || input.modifiers.ctrl {
            if input.key_pressed(Key::Z) {
                if input.modifiers.shift {
                    if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                        buf.redo();
                    }
                } else if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    buf.undo();
                }
            } else if input.key_pressed(Key::Y) {
                if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    buf.redo();
                }
            } else if input.key_pressed(Key::S) {
                if let Some(buf) = self.open_buffers.get_mut(self.active_buffer_idx) {
                    let _ = buf.save();
                    self.status_msg = format!("Saved: {}", buf.file_name);
                }
            } else if input.key_pressed(Key::Backtick) {
                self.terminal_open = !self.terminal_open;
            }
            return;
        }

        // Active buffer interactive editing
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
                buf.insert_char('\n');
            } else if input.key_pressed(Key::Tab) {
                buf.insert_str("    ");
            } else {
                for event in &input.events {
                    if let egui::Event::Text(text) = event {
                        if !text.is_empty() && !input.modifiers.command && !input.modifiers.ctrl {
                            buf.insert_str(text);
                        }
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
            .exact_height(30.0)
            .frame(egui::Frame::none().fill(COLOR_SIDEBAR_HEADER))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label(RichText::new("Code (Native Rust)").color(COLOR_TEXT_PRIMARY).strong());
                    ui.add_space(20.0);
                    if ui.button("Open Folder...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.explorer.load_directory(&path);
                        }
                    }
                    if ui.button("Open File...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.open_file(&path);
                        }
                    }
                    if ui.button(if self.terminal_open { "Hide Terminal (Ctrl+`)" } else { "Show Terminal (Ctrl+`)" }).clicked() {
                        self.terminal_open = !self.terminal_open;
                    }
                });
            });

        // Bottom Status Bar
        egui::TopBottomPanel::bottom("status_bar")
            .exact_height(22.0)
            .frame(egui::Frame::none().fill(COLOR_STATUS_BAR))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label(RichText::new("⎇ main*").color(Color32::WHITE).size(11.0));
                    ui.add_space(15.0);
                    ui.label(RichText::new(&self.status_msg).color(Color32::WHITE).size(11.0));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        ui.label(RichText::new("UTF-8").color(Color32::WHITE).size(11.0));
                        ui.add_space(10.0);
                        if let Some(buf) = self.open_buffers.get(self.active_buffer_idx) {
                            let ext = buf.get_extension().to_uppercase();
                            ui.label(RichText::new(ext).color(Color32::WHITE).size(11.0));
                            ui.add_space(10.0);
                            let pos_text = format!("Ln {}, Col {}", buf.cursor_line() + 1, buf.cursor_col() + 1);
                            ui.label(RichText::new(pos_text).color(Color32::WHITE).size(11.0));
                        }
                    });
                });
            });

        // Left Activity Bar
        egui::SidePanel::left("activity_bar")
            .exact_width(48.0)
            .resizable(false)
            .frame(egui::Frame::none().fill(COLOR_ACTIVITY_BAR))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    let tabs = [
                        (ActivityBarTab::Explorer, "📁"),
                        (ActivityBarTab::Search, "🔍"),
                        (ActivityBarTab::SourceControl, "🔀"),
                        (ActivityBarTab::Extensions, "🧩"),
                    ];
                    for (tab, icon) in tabs {
                        let is_active = self.active_tab == tab;
                        let text = RichText::new(icon).size(20.0).color(if is_active {
                            Color32::WHITE
                        } else {
                            COLOR_TEXT_MUTED
                        });
                        if ui.add(egui::Button::new(text).frame(false)).clicked() {
                            self.active_tab = tab;
                        }
                        ui.add_space(12.0);
                    }

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        let text = RichText::new("⚙").size(20.0).color(COLOR_TEXT_MUTED);
                        if ui.add(egui::Button::new(text).frame(false)).clicked() {
                            self.active_tab = ActivityBarTab::Settings;
                        }
                    });
                });
            });

        // Primary Sidebar (Folder Tree / Explorer)
        egui::SidePanel::left("primary_sidebar")
            .default_width(220.0)
            .width_range(150.0..=400.0)
            .frame(egui::Frame::none().fill(COLOR_SIDEBAR_BG))
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    let title = match self.active_tab {
                        ActivityBarTab::Explorer => "EXPLORER",
                        ActivityBarTab::Search => "SEARCH",
                        ActivityBarTab::SourceControl => "SOURCE CONTROL",
                        ActivityBarTab::Extensions => "EXTENSIONS",
                        ActivityBarTab::Settings => "SETTINGS",
                    };
                    ui.label(RichText::new(title).size(11.0).color(COLOR_TEXT_MUTED).strong());
                });
                ui.separator();

                if self.active_tab == ActivityBarTab::Explorer {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let mut clicked_file = None;
                        for entry in &self.explorer.entries {
                            let icon = if entry.is_dir { "📁 " } else { "📄 " };
                            let label = format!("{}{}", icon, entry.name);
                            if ui.selectable_label(false, label).clicked() {
                                if !entry.is_dir {
                                    clicked_file = Some(entry.path.clone());
                                }
                            }
                        }
                        if let Some(path) = clicked_file {
                            self.open_file(&path);
                        }
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(RichText::new("Panel coming soon").color(COLOR_TEXT_MUTED));
                    });
                }
            });

        // Bottom Integrated Terminal Panel
        if self.terminal_open {
            egui::TopBottomPanel::bottom("terminal_panel")
                .default_height(200.0)
                .height_range(100.0..=400.0)
                .resizable(true)
                .frame(egui::Frame::none().fill(COLOR_EDITOR_BG))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(12.0);
                        ui.label(RichText::new("TERMINAL").color(COLOR_TEXT_PRIMARY).strong().size(11.0));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("✕").clicked() {
                                self.terminal_open = false;
                            }
                        });
                    });
                    ui.separator();

                    if let Some(term) = &self.terminal {
                        let text = term.output.lock().unwrap_or_else(|e| e.into_inner()).clone();
                        egui::ScrollArea::vertical()
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                ui.label(RichText::new(text).font(FontId::monospace(12.0)).color(COLOR_TEXT_PRIMARY));
                            });
                    }
                });
        }

        // Central Area: Tab Bar + Editor Buffer with Syntax Highlighting
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(COLOR_EDITOR_BG))
            .show(ctx, |ui| {
                // Editor Tabs Header
                ui.horizontal(|ui| {
                    for (i, buf) in self.open_buffers.iter().enumerate() {
                        let is_active = i == self.active_buffer_idx;
                        let tab_bg = if is_active { COLOR_TAB_ACTIVE } else { COLOR_TAB_INACTIVE };
                        let title = format!("{} {}", buf.file_name, if buf.is_dirty { "●" } else { "" });

                        egui::Frame::none()
                            .fill(tab_bg)
                            .inner_margin(egui::Margin::symmetric(12.0, 6.0))
                            .show(ui, |ui| {
                                let label_color = if is_active { Color32::WHITE } else { COLOR_TEXT_MUTED };
                                if ui.selectable_label(is_active, RichText::new(title).color(label_color)).clicked() {
                                    self.active_buffer_idx = i;
                                }
                            });
                    }
                });

                ui.separator();

                // Editor Text Buffer Area with Line Numbers & Syntax Highlighting
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if let Some(buf) = self.open_buffers.get(self.active_buffer_idx) {
                            let line_count = buf.line_count();
                            let ext = buf.get_extension().to_string();
                            let font_id = FontId::monospace(13.0);
                            let cur_line = buf.cursor_line();
                            let cur_col = buf.cursor_col();

                            ui.horizontal_top(|ui| {
                                // Gutter / Line Numbers
                                ui.vertical(|ui| {
                                    ui.set_width(40.0);
                                    for idx in 0..line_count {
                                        let num_str = format!("{:>3}", idx + 1);
                                        let col = if idx == cur_line { Color32::WHITE } else { COLOR_LINE_NUMBERS };
                                        ui.label(RichText::new(num_str).font(font_id.clone()).color(col));
                                    }
                                });

                                ui.separator();

                                // Highlighted Code Lines with Cursor
                                ui.vertical(|ui| {
                                    for idx in 0..line_count {
                                        let raw_line = buf.line_str(idx).unwrap_or_default();
                                        let tokens = self.syntax_highlighter.highlight_line(&raw_line, &ext);

                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing.x = 0.0;
                                            let mut col_offset = 0;

                                            for (color, token_text) in tokens {
                                                let clean_text = token_text.trim_end_matches(['\r', '\n']);
                                                if clean_text.is_empty() {
                                                    continue;
                                                }

                                                let token_len = clean_text.chars().count();
                                                if idx == cur_line && cur_col >= col_offset && cur_col < col_offset + token_len {
                                                    // Render token with cursor indicator
                                                    let split_pos = cur_col - col_offset;
                                                    let before: String = clean_text.chars().take(split_pos).collect();
                                                    let at_cursor: String = clean_text.chars().skip(split_pos).take(1).collect();
                                                    let after: String = clean_text.chars().skip(split_pos + 1).collect();

                                                    if !before.is_empty() {
                                                        ui.label(RichText::new(before).font(font_id.clone()).color(color));
                                                    }
                                                    ui.label(RichText::new(if at_cursor.is_empty() { " " } else { &at_cursor })
                                                        .font(font_id.clone())
                                                        .background_color(Color32::from_rgb(0, 122, 204))
                                                        .color(Color32::WHITE));
                                                    if !after.is_empty() {
                                                        ui.label(RichText::new(after).font(font_id.clone()).color(color));
                                                    }
                                                } else {
                                                    ui.label(RichText::new(clean_text).font(font_id.clone()).color(color));
                                                }
                                                col_offset += token_len;
                                            }

                                            // Trailing cursor on end of line
                                            if idx == cur_line && cur_col >= col_offset {
                                                ui.label(RichText::new(" ")
                                                    .font(font_id.clone())
                                                    .background_color(Color32::from_rgb(0, 122, 204)));
                                            }
                                        });
                                    }
                                });
                            });
                        }
                    });
            });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(1024.0, 720.0))
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
