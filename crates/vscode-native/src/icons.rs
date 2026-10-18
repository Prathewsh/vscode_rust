use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Ui, Vec2};

pub struct MaterialIcon;

impl MaterialIcon {
    /// Renders official VS Code Material / Seti vector file and folder icons
    pub fn render_file_icon(ui: &mut Ui, name: &str, is_dir: bool, is_expanded: bool) {
        if is_dir {
            let folder_color = Color32::from_rgb(229, 169, 60); // Material folder yellow
            let (rect, _) = ui.allocate_exact_size(Vec2::new(16.0, 14.0), egui::Sense::hover());
            let painter = ui.painter();

            // Draw folder tab
            let tab_rect = Rect::from_min_size(rect.min, Vec2::new(6.5, 3.5));
            painter.rect_filled(tab_rect, 1.0, folder_color);

            // Draw folder body
            let body_rect = Rect::from_min_max(
                Pos2::new(rect.min.x, rect.min.y + 2.5),
                rect.max,
            );
            if is_expanded {
                painter.rect_filled(body_rect, 1.5, folder_color);
                let inside_rect = Rect::from_min_max(
                    Pos2::new(rect.min.x + 2.0, rect.min.y + 4.5),
                    Pos2::new(rect.max.x - 2.0, rect.max.y - 1.0),
                );
                painter.rect_filled(inside_rect, 1.0, Color32::from_rgb(160, 110, 30));
            } else {
                painter.rect_filled(body_rect, 1.5, folder_color);
            }
            return;
        }

        let (rect, _) = ui.allocate_exact_size(Vec2::new(16.0, 14.0), egui::Sense::hover());
        let painter = ui.painter();
        let center = rect.center();

        if name.ends_with(".rs") {
            // Authentic Rust Gear / Cog
            let rust_orange = Color32::from_rgb(247, 76, 0);
            let stroke = Stroke::new(1.2, rust_orange);
            for i in 0..8 {
                let angle = (i as f32) * std::f32::consts::PI / 4.0;
                let p1 = Pos2::new(center.x + angle.cos() * 4.5, center.y + angle.sin() * 4.5);
                let p2 = Pos2::new(center.x + angle.cos() * 6.5, center.y + angle.sin() * 6.5);
                painter.line_segment([p1, p2], Stroke::new(1.8, rust_orange));
            }
            painter.circle_stroke(center, 4.5, stroke);
            painter.text(center, egui::Align2::CENTER_CENTER, "R", egui::FontId::monospace(8.0), Color32::WHITE);
            return;
        }

        if name.ends_with(".md") || name.ends_with(".txt") {
            // Official Markdown Icon
            let md_blue = Color32::from_rgb(56, 189, 248);
            let border_rect = Rect::from_center_size(center, Vec2::new(15.0, 11.0));
            painter.rect_stroke(border_rect, 2.0, Stroke::new(1.0, md_blue));
            let m_x = center.x - 3.5;
            painter.line_segment([Pos2::new(m_x - 2.0, center.y + 3.0), Pos2::new(m_x - 2.0, center.y - 3.0)], Stroke::new(1.2, md_blue));
            painter.line_segment([Pos2::new(m_x - 2.0, center.y - 3.0), Pos2::new(m_x, center.y)], Stroke::new(1.2, md_blue));
            painter.line_segment([Pos2::new(m_x, center.y), Pos2::new(m_x + 2.0, center.y - 3.0)], Stroke::new(1.2, md_blue));
            painter.line_segment([Pos2::new(m_x + 2.0, center.y - 3.0), Pos2::new(m_x + 2.0, center.y + 3.0)], Stroke::new(1.2, md_blue));
            let a_x = center.x + 3.5;
            painter.line_segment([Pos2::new(a_x, center.y - 3.0), Pos2::new(a_x, center.y + 3.0)], Stroke::new(1.2, md_blue));
            painter.line_segment([Pos2::new(a_x - 2.0, center.y + 1.0), Pos2::new(a_x, center.y + 3.0)], Stroke::new(1.2, md_blue));
            painter.line_segment([Pos2::new(a_x + 2.0, center.y + 1.0), Pos2::new(a_x, center.y + 3.0)], Stroke::new(1.2, md_blue));
            return;
        }

        if name == ".gitignore" || name.contains("git") {
            // Git diamond logo (rhombus rotated 45 deg)
            let git_orange = Color32::from_rgb(240, 80, 50);
            let p_top = Pos2::new(center.x, center.y - 5.5);
            let p_right = Pos2::new(center.x + 5.5, center.y);
            let p_bottom = Pos2::new(center.x, center.y + 5.5);
            let p_left = Pos2::new(center.x - 5.5, center.y);
            let stroke = Stroke::new(1.4, git_orange);
            painter.line_segment([p_top, p_right], stroke);
            painter.line_segment([p_right, p_bottom], stroke);
            painter.line_segment([p_bottom, p_left], stroke);
            painter.line_segment([p_left, p_top], stroke);
            painter.circle_filled(Pos2::new(center.x - 2.0, center.y - 1.5), 1.5, git_orange);
            painter.circle_filled(Pos2::new(center.x + 1.5, center.y - 2.0), 1.5, git_orange);
            painter.circle_filled(Pos2::new(center.x - 2.0, center.y + 2.0), 1.5, git_orange);
            painter.line_segment([Pos2::new(center.x - 2.0, center.y - 1.5), Pos2::new(center.x - 2.0, center.y + 2.0)], Stroke::new(1.0, git_orange));
            painter.line_segment([Pos2::new(center.x - 2.0, center.y), Pos2::new(center.x + 1.5, center.y - 2.0)], Stroke::new(1.0, git_orange));
            return;
        }

        if name.ends_with(".json") {
            // Yellow { } curly brackets
            let json_yellow = Color32::from_rgb(250, 204, 21);
            painter.text(center, egui::Align2::CENTER_CENTER, "{ }", egui::FontId::monospace(11.0), json_yellow);
            return;
        }

        if name == "Cargo.toml" || name.ends_with(".toml") {
            let gold = Color32::from_rgb(229, 192, 123);
            let doc_rect = Rect::from_center_size(center, Vec2::new(12.0, 13.0));
            painter.rect_stroke(doc_rect, 1.5, Stroke::new(1.2, gold));
            painter.circle_stroke(center, 2.5, Stroke::new(1.0, gold));
            return;
        }

        if name.ends_with(".lock") {
            let lock_gray = Color32::from_rgb(160, 160, 175);
            let shackle_c = Pos2::new(center.x, center.y - 2.0);
            painter.circle_stroke(shackle_c, 3.2, Stroke::new(1.3, lock_gray));
            let body_rect = Rect::from_center_size(Pos2::new(center.x, center.y + 2.0), Vec2::new(10.0, 7.5));
            painter.rect_filled(body_rect, 1.5, lock_gray);
            painter.circle_filled(Pos2::new(center.x, center.y + 1.5), 1.0, Color32::from_rgb(24, 24, 27));
            return;
        }

        if name.ends_with(".ts") || name.ends_with(".tsx") {
            let r = Rect::from_center_size(center, Vec2::new(14.0, 12.0));
            painter.rect_filled(r, 2.0, Color32::from_rgb(49, 120, 198));
            painter.text(center, egui::Align2::CENTER_CENTER, "TS", egui::FontId::monospace(8.0), Color32::WHITE);
            return;
        }

        if name.ends_with(".js") || name.ends_with(".jsx") {
            let r = Rect::from_center_size(center, Vec2::new(14.0, 12.0));
            painter.rect_filled(r, 2.0, Color32::from_rgb(247, 223, 30));
            painter.text(center, egui::Align2::CENTER_CENTER, "JS", egui::FontId::monospace(8.0), Color32::from_rgb(20, 20, 20));
            return;
        }

        if name.ends_with(".sh") || name.ends_with(".bash") || name.ends_with(".zsh") {
            let r = Rect::from_center_size(center, Vec2::new(14.0, 12.0));
            painter.rect_filled(r, 2.0, Color32::from_rgb(16, 185, 129));
            painter.text(center, egui::Align2::CENTER_CENTER, ">_", egui::FontId::monospace(8.0), Color32::WHITE);
            return;
        }

        if name.ends_with(".html") {
            painter.text(center, egui::Align2::CENTER_CENTER, "<>", egui::FontId::monospace(10.0), Color32::from_rgb(228, 77, 38));
            return;
        }

        if name.ends_with(".css") {
            painter.text(center, egui::Align2::CENTER_CENTER, "#", egui::FontId::monospace(11.0), Color32::from_rgb(38, 77, 228));
            return;
        }

        if name.ends_with(".png") || name.ends_with(".jpg") || name.ends_with(".svg") || name.ends_with(".ico") {
            let purple = Color32::from_rgb(168, 85, 247);
            let r = Rect::from_center_size(center, Vec2::new(13.0, 11.0));
            painter.rect_stroke(r, 1.0, Stroke::new(1.1, purple));
            painter.circle_filled(Pos2::new(center.x - 2.5, center.y - 2.0), 1.2, purple);
            let p1 = Pos2::new(center.x - 4.5, center.y + 3.5);
            let p2 = Pos2::new(center.x, center.y - 0.5);
            let p3 = Pos2::new(center.x + 4.5, center.y + 3.5);
            painter.line_segment([p1, p2], Stroke::new(1.0, purple));
            painter.line_segment([p2, p3], Stroke::new(1.0, purple));
            return;
        }

        // Default document icon with folded corner
        let doc_color = Color32::from_rgb(148, 163, 184);
        let doc_rect = Rect::from_center_size(center, Vec2::new(11.0, 13.0));
        painter.rect_stroke(doc_rect, 1.0, Stroke::new(1.1, doc_color));
        painter.line_segment([Pos2::new(center.x - 3.0, center.y - 1.5), Pos2::new(center.x + 3.0, center.y - 1.5)], Stroke::new(1.0, doc_color));
        painter.line_segment([Pos2::new(center.x - 3.0, center.y + 1.5), Pos2::new(center.x + 1.0, center.y + 1.5)], Stroke::new(1.0, doc_color));
    }

    /// Renders a crisp vector close 'x' button
    pub fn close_button(ui: &mut Ui, size: f32) -> bool {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::click());
        let hovered = response.hovered();
        let painter = ui.painter();

        if hovered {
            painter.rect_filled(rect, 3.0, Color32::from_rgba_unmultiplied(230, 50, 50, 60));
        }

        let cross_color = if hovered { Color32::WHITE } else { Color32::from_rgb(160, 160, 170) };
        let pad = size * 0.28;
        let stroke = Stroke::new(1.3, cross_color);

        painter.line_segment(
            [
                Pos2::new(rect.min.x + pad, rect.min.y + pad),
                Pos2::new(rect.max.x - pad, rect.max.y - pad),
            ],
            stroke,
        );
        painter.line_segment(
            [
                Pos2::new(rect.max.x - pad, rect.min.y + pad),
                Pos2::new(rect.min.x + pad, rect.max.y - pad),
            ],
            stroke,
        );

        response.clicked()
    }

    /// Renders Activity Bar vector icons
    pub fn activity_bar_icon(ui: &mut Ui, id: &str, is_active: bool, tooltip: &str) -> bool {
        let (rect, response) = ui.allocate_exact_size(Vec2::new(48.0, 42.0), egui::Sense::click());
        let response = response.on_hover_text(tooltip);
        let hovered = response.hovered();
        let color = if is_active {
            Color32::WHITE
        } else if hovered {
            Color32::from_rgb(220, 220, 225)
        } else {
            Color32::from_rgb(115, 115, 125)
        };

        let painter = ui.painter();
        let center = rect.center();
        let stroke = Stroke::new(1.4, color);

        match id {
            "explorer" => {
                // Two overlapping document outlines
                let r1 = Rect::from_center_size(Pos2::new(center.x - 3.0, center.y - 2.5), Vec2::new(13.0, 17.0));
                painter.rect_stroke(r1, 1.0, stroke);
                let r2 = Rect::from_center_size(Pos2::new(center.x + 3.0, center.y + 2.5), Vec2::new(13.0, 17.0));
                painter.rect_filled(r2, 1.0, Color32::from_rgb(20, 20, 22)); // occlude back sheet
                painter.rect_stroke(r2, 1.0, stroke);
            }
            "search" => {
                // Magnifying glass at 45 degrees
                let c = Pos2::new(center.x - 3.0, center.y - 3.0);
                painter.circle_stroke(c, 6.0, stroke);
                painter.line_segment(
                    [
                        Pos2::new(center.x + 2.0, center.y + 2.0),
                        Pos2::new(center.x + 8.5, center.y + 8.5),
                    ],
                    Stroke::new(2.0, color),
                );
            }
            "git" => {
                // Git branch: vertical stem with circle, curved branch with circle
                let top_node = Pos2::new(center.x + 4.0, center.y - 5.0);
                let bot_left = Pos2::new(center.x - 5.0, center.y + 5.0);
                let top_left = Pos2::new(center.x - 5.0, center.y - 5.0);

                painter.line_segment([top_left, bot_left], stroke);
                painter.line_segment([Pos2::new(center.x - 5.0, center.y + 1.0), top_node], stroke);

                painter.circle_filled(top_node, 2.5, color);
                painter.circle_filled(bot_left, 2.5, color);
                painter.circle_filled(top_left, 2.5, color);

                // Blue badge "1K+" overlapping bottom-center like in screenshot
                let badge_rect = Rect::from_center_size(
                    Pos2::new(center.x + 3.0, center.y + 6.0),
                    Vec2::new(20.0, 11.5),
                );
                painter.rect_filled(badge_rect, 4.0, Color32::from_rgb(0, 120, 215));
                painter.text(
                    badge_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "1K+",
                    egui::FontId::monospace(8.0),
                    Color32::WHITE,
                );
            }
            "debug" => {
                // Play triangle with bug attached
                let p1 = Pos2::new(center.x - 5.0, center.y - 7.0);
                let p2 = Pos2::new(center.x + 6.0, center.y);
                let p3 = Pos2::new(center.x - 5.0, center.y + 7.0);
                painter.line_segment([p1, p2], stroke);
                painter.line_segment([p2, p3], stroke);
                painter.line_segment([p3, p1], stroke);

                // Small bug at bottom right
                let bug_c = Pos2::new(center.x + 1.0, center.y + 4.0);
                painter.circle_filled(bug_c, 2.0, color);
                painter.line_segment([Pos2::new(bug_c.x - 2.5, bug_c.y + 1.0), Pos2::new(bug_c.x + 2.5, bug_c.y + 1.0)], Stroke::new(1.0, color));
                painter.line_segment([Pos2::new(bug_c.x - 2.0, bug_c.y - 1.5), Pos2::new(bug_c.x + 2.0, bug_c.y - 1.5)], Stroke::new(1.0, color));
            }
            "remote" => {
                // Terminal monitor screen
                let monitor = Rect::from_center_size(Pos2::new(center.x, center.y - 1.5), Vec2::new(18.0, 13.0));
                painter.rect_stroke(monitor, 1.5, stroke);
                // Prompt < > inside screen
                painter.line_segment([Pos2::new(center.x - 4.0, center.y - 3.5), Pos2::new(center.x - 1.5, center.y - 1.5)], stroke);
                painter.line_segment([Pos2::new(center.x - 1.5, center.y - 1.5), Pos2::new(center.x - 4.0, center.y + 0.5)], stroke);
                painter.line_segment([Pos2::new(center.x, center.y + 0.5), Pos2::new(center.x + 4.0, center.y + 0.5)], stroke);
                // Stand base
                painter.line_segment([Pos2::new(center.x, center.y + 5.0), Pos2::new(center.x, center.y + 7.5)], stroke);
                painter.line_segment([Pos2::new(center.x - 4.0, center.y + 7.5), Pos2::new(center.x + 4.0, center.y + 7.5)], stroke);
            }
            "extensions" => {
                // 4 squares: 3 in L shape, 1 floating top-right
                let s = 5.5;
                let tl = Rect::from_center_size(Pos2::new(center.x - 4.5, center.y - 4.5), Vec2::splat(s));
                let bl = Rect::from_center_size(Pos2::new(center.x - 4.5, center.y + 4.5), Vec2::splat(s));
                let br = Rect::from_center_size(Pos2::new(center.x + 4.5, center.y + 4.5), Vec2::splat(s));
                let tr = Rect::from_center_size(Pos2::new(center.x + 6.5, center.y - 6.5), Vec2::splat(s));

                painter.rect_stroke(tl, 1.0, stroke);
                painter.rect_stroke(bl, 1.0, stroke);
                painter.rect_stroke(br, 1.0, stroke);
                painter.rect_stroke(tr, 1.0, stroke);
            }
            "package" => {
                // Authentic 3D Isometric Cube / Box with top, left, and right faces
                let c = center;
                let top = Pos2::new(c.x, c.y - 8.5);
                let tr = Pos2::new(c.x + 7.5, c.y - 4.2);
                let tl = Pos2::new(c.x - 7.5, c.y - 4.2);
                let br = Pos2::new(c.x + 7.5, c.y + 4.2);
                let bl = Pos2::new(c.x - 7.5, c.y + 4.2);
                let bot = Pos2::new(c.x, c.y + 8.5);

                // Top rhombus
                painter.line_segment([top, tr], stroke);
                painter.line_segment([tr, c], stroke);
                painter.line_segment([c, tl], stroke);
                painter.line_segment([tl, top], stroke);

                // Vertical edges
                painter.line_segment([tl, bl], stroke);
                painter.line_segment([c, bot], stroke);
                painter.line_segment([tr, br], stroke);

                // Bottom edges
                painter.line_segment([bl, bot], stroke);
                painter.line_segment([bot, br], stroke);
            }
            "settings" => {
                painter.circle_stroke(center, 6.0, stroke);
                painter.circle_filled(center, 2.0, color);
                for i in 0..6 {
                    let angle = (i as f32) * std::f32::consts::PI / 3.0;
                    let p1 = Pos2::new(center.x + angle.cos() * 6.0, center.y + angle.sin() * 6.0);
                    let p2 = Pos2::new(center.x + angle.cos() * 9.0, center.y + angle.sin() * 9.0);
                    painter.line_segment([p1, p2], Stroke::new(2.0, color));
                }
            }
            _ => {}
        }

        // Active indicator on left edge (Crisp white bar as seen in screenshot)
        if is_active {
            let bar = Rect::from_min_max(
                Pos2::new(rect.min.x, rect.min.y + 6.0),
                Pos2::new(rect.min.x + 2.0, rect.max.y - 6.0),
            );
            painter.rect_filled(bar, 0.5, Color32::WHITE);
        }

        response.clicked()
    }

    /// Renders search header action buttons (refresh, clear, new editor, collapse, open editor)
    pub fn search_header_button(ui: &mut Ui, id: &str, tooltip: &str) -> bool {
        let (rect, response) = ui.allocate_exact_size(Vec2::new(20.0, 20.0), egui::Sense::click());
        let response = response.on_hover_text(tooltip);
        let hovered = response.hovered();
        let color = if hovered { Color32::WHITE } else { Color32::from_rgb(140, 140, 150) };
        let stroke = Stroke::new(1.3, color);
        let center = rect.center();
        let painter = ui.painter();

        if hovered {
            painter.rect_filled(rect, 3.0, Color32::from_rgb(45, 45, 52));
        }

        match id {
            "refresh" => {
                // Clockwise circular arrow arc
                let r = 5.0;
                let num_segments = 12;
                let start_angle = 0.5_f32;
                let end_angle = 5.6_f32;
                for i in 0..num_segments {
                    let a1 = start_angle + (end_angle - start_angle) * (i as f32 / num_segments as f32);
                    let a2 = start_angle + (end_angle - start_angle) * ((i + 1) as f32 / num_segments as f32);
                    let p1 = Pos2::new(center.x + a1.cos() * r, center.y + a1.sin() * r);
                    let p2 = Pos2::new(center.x + a2.cos() * r, center.y + a2.sin() * r);
                    painter.line_segment([p1, p2], stroke);
                }
                // Arrow head
                let tip = Pos2::new(center.x + end_angle.cos() * r, center.y + end_angle.sin() * r);
                painter.line_segment([tip, Pos2::new(tip.x - 3.5, tip.y - 0.5)], stroke);
                painter.line_segment([tip, Pos2::new(tip.x - 0.5, tip.y + 3.5)], stroke);
            }
            "clear" => {
                // 3 horizontal lines with 'x' on right
                painter.line_segment([Pos2::new(center.x - 6.0, center.y - 3.5), Pos2::new(center.x + 0.5, center.y - 3.5)], stroke);
                painter.line_segment([Pos2::new(center.x - 6.0, center.y), Pos2::new(center.x + 0.5, center.y)], stroke);
                painter.line_segment([Pos2::new(center.x - 6.0, center.y + 3.5), Pos2::new(center.x + 0.5, center.y + 3.5)], stroke);
                // Small cross on right
                let xc = Pos2::new(center.x + 4.5, center.y);
                painter.line_segment([Pos2::new(xc.x - 2.0, xc.y - 2.0), Pos2::new(xc.x + 2.0, xc.y + 2.0)], stroke);
                painter.line_segment([Pos2::new(xc.x + 2.0, xc.y - 2.0), Pos2::new(xc.x - 2.0, xc.y + 2.0)], stroke);
            }
            "new_search_editor" => {
                // Page outline with '+' in bottom right
                let page = Rect::from_min_size(Pos2::new(center.x - 6.5, center.y - 6.5), Vec2::new(9.0, 12.0));
                painter.rect_stroke(page, 1.0, stroke);
                // Plus
                let pc = Pos2::new(center.x + 4.0, center.y + 3.5);
                painter.line_segment([Pos2::new(pc.x - 3.0, pc.y), Pos2::new(pc.x + 3.0, pc.y)], Stroke::new(1.4, color));
                painter.line_segment([Pos2::new(pc.x, pc.y - 3.0), Pos2::new(pc.x, pc.y + 3.0)], Stroke::new(1.4, color));
            }
            "collapse_all" => {
                // Horizontal lines staggered
                painter.line_segment([Pos2::new(center.x - 6.0, center.y - 4.0), Pos2::new(center.x + 6.0, center.y - 4.0)], stroke);
                painter.line_segment([Pos2::new(center.x - 4.0, center.y), Pos2::new(center.x + 6.0, center.y)], stroke);
                painter.line_segment([Pos2::new(center.x - 2.0, center.y + 4.0), Pos2::new(center.x + 6.0, center.y + 4.0)], stroke);
            }
            "open_editor" => {
                // Two overlapping panes
                let r1 = Rect::from_min_size(Pos2::new(center.x - 6.0, center.y - 6.0), Vec2::new(8.5, 9.5));
                painter.rect_stroke(r1, 1.0, stroke);
                let r2 = Rect::from_min_size(Pos2::new(center.x - 2.0, center.y - 2.5), Vec2::new(8.5, 9.5));
                painter.rect_filled(r2, 1.0, Color32::from_rgb(24, 24, 27));
                painter.rect_stroke(r2, 1.0, stroke);
            }
            _ => {}
        }

        response.clicked()
    }

    /// Renders a sharp vector chevron button for toggling Search & Replace
    pub fn chevron_button(ui: &mut Ui, is_expanded: bool, tooltip: &str) -> bool {
        let (rect, response) = ui.allocate_exact_size(Vec2::new(14.0, 18.0), egui::Sense::click());
        let response = response.on_hover_text(tooltip);
        let hovered = response.hovered();
        let color = if hovered { Color32::WHITE } else { Color32::from_rgb(140, 140, 150) };
        let stroke = Stroke::new(1.3, color);
        let center = rect.center();
        let painter = ui.painter();

        if is_expanded {
            // Down chevron 'v'
            painter.line_segment([Pos2::new(center.x - 4.0, center.y - 2.0), Pos2::new(center.x, center.y + 2.0)], stroke);
            painter.line_segment([Pos2::new(center.x, center.y + 2.0), Pos2::new(center.x + 4.0, center.y - 2.0)], stroke);
        } else {
            // Right chevron '>'
            painter.line_segment([Pos2::new(center.x - 2.0, center.y - 4.0), Pos2::new(center.x + 2.0, center.y)], stroke);
            painter.line_segment([Pos2::new(center.x + 2.0, center.y), Pos2::new(center.x - 2.0, center.y + 4.0)], stroke);
        }

        response.clicked()
    }

    /// Renders Replace All vector button
    pub fn replace_all_button(ui: &mut Ui, tooltip: &str) -> bool {
        let (rect, response) = ui.allocate_exact_size(Vec2::new(20.0, 18.0), egui::Sense::click());
        let response = response.on_hover_text(tooltip);
        let hovered = response.hovered();
        let color = if hovered { Color32::WHITE } else { Color32::from_rgb(140, 140, 150) };
        let painter = ui.painter();

        if hovered {
            painter.rect_filled(rect, 2.0, Color32::from_rgb(50, 50, 55));
        }

        // Draw "AB" with replacement arrow/bar
        painter.text(
            Pos2::new(rect.min.x + 3.0, rect.center().y - 3.0),
            egui::Align2::LEFT_CENTER,
            "ab",
            egui::FontId::monospace(8.0),
            color,
        );

        // Arrow underneath / across
        let stroke = Stroke::new(1.2, color);
        let y = rect.max.y - 4.5;
        painter.line_segment([Pos2::new(rect.min.x + 3.0, y), Pos2::new(rect.max.x - 4.0, y)], stroke);
        painter.line_segment([Pos2::new(rect.max.x - 6.5, y - 2.0), Pos2::new(rect.max.x - 4.0, y)], stroke);
        painter.line_segment([Pos2::new(rect.max.x - 6.5, y + 2.0), Pos2::new(rect.max.x - 4.0, y)], stroke);

        response.clicked()
    }

    /// Renders search details toggle button `...`
    pub fn details_button(ui: &mut Ui, active: bool, tooltip: &str) -> bool {
        let (rect, response) = ui.allocate_exact_size(Vec2::new(18.0, 14.0), egui::Sense::click());
        let response = response.on_hover_text(tooltip);
        let hovered = response.hovered();
        let color = if active {
            Color32::from_rgb(56, 189, 248) // sky blue
        } else if hovered {
            Color32::WHITE
        } else {
            Color32::from_rgb(130, 130, 140)
        };
        let painter = ui.painter();

        if hovered {
            painter.rect_filled(rect, 2.0, Color32::from_rgb(45, 45, 52));
        }

        let cy = rect.center().y;
        let cx = rect.center().x;
        painter.circle_filled(Pos2::new(cx - 4.0, cy), 1.2, color);
        painter.circle_filled(Pos2::new(cx, cy), 1.2, color);
        painter.circle_filled(Pos2::new(cx + 4.0, cy), 1.2, color);

        response.clicked()
    }

    /// Renders a toggle button inside search/replace input box (Aa, ab, .*, AB)
    pub fn search_toggle(ui: &mut Ui, active: &mut bool, label: &str, tooltip: &str) {
        let (rect, response) = ui.allocate_exact_size(Vec2::new(20.0, 18.0), egui::Sense::click());
        let response = response.on_hover_text(tooltip);
        if response.clicked() {
            *active = !*active;
        }

        let painter = ui.painter();
        if *active {
            painter.rect_filled(rect, 2.0, Color32::from_rgb(0, 122, 204));
            painter.rect_stroke(rect, 2.0, Stroke::new(1.0, Color32::from_rgb(0, 150, 255)));
        } else if response.hovered() {
            painter.rect_filled(rect, 2.0, Color32::from_rgb(50, 50, 56));
        }

        let text_color = if *active {
            Color32::WHITE
        } else if response.hovered() {
            Color32::from_rgb(230, 230, 235)
        } else {
            Color32::from_rgb(140, 140, 150)
        };

        if label == "ab" {
            // Whole word has text 'ab' and an underline beneath it like in screenshot
            painter.text(
                Pos2::new(rect.center().x, rect.center().y - 1.5),
                egui::Align2::CENTER_CENTER,
                "ab",
                egui::FontId::monospace(10.0),
                text_color,
            );
            let line_y = rect.max.y - 3.0;
            painter.line_segment(
                [Pos2::new(rect.min.x + 4.0, line_y), Pos2::new(rect.max.x - 4.0, line_y)],
                Stroke::new(1.3, text_color),
            );
        } else if label == ".*" {
            // Regex has a dot at bottom-left and an asterisk at top-right
            painter.text(
                Pos2::new(rect.center().x - 3.0, rect.center().y + 2.0),
                egui::Align2::CENTER_CENTER,
                ".",
                egui::FontId::monospace(13.0),
                text_color,
            );
            painter.text(
                Pos2::new(rect.center().x + 2.5, rect.center().y - 1.5),
                egui::Align2::CENTER_CENTER,
                "*",
                egui::FontId::monospace(12.0),
                text_color,
            );
        } else {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::monospace(10.5),
                text_color,
            );
        }
    }

    /// Renders terminal instance icon [>_]
    pub fn terminal_session_icon(ui: &mut Ui, is_active: bool) {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(18.0, 14.0), egui::Sense::hover());
        let painter = ui.painter();
        let color = if is_active { Color32::WHITE } else { Color32::from_rgb(150, 150, 160) };
        let box_rect = Rect::from_center_size(rect.center(), Vec2::new(15.0, 11.5));
        painter.rect_stroke(box_rect, 1.5, Stroke::new(1.0, color));
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            ">_",
            egui::FontId::monospace(8.0),
            color,
        );
    }

    /// Renders terminal action buttons: "+ v", "...", "maximize", "close"
    pub fn terminal_action_button(ui: &mut Ui, id: &str, tooltip: &str) -> bool {
        let size = if id == "add" { Vec2::new(26.0, 20.0) } else { Vec2::new(20.0, 20.0) };
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
        let response = response.on_hover_text(tooltip);
        let hovered = response.hovered();
        let color = if hovered { Color32::WHITE } else { Color32::from_rgb(140, 140, 150) };
        let stroke = Stroke::new(1.2, color);
        let center = rect.center();
        let painter = ui.painter();

        if hovered {
            painter.rect_filled(rect, 3.0, Color32::from_rgb(45, 45, 52));
        }

        match id {
            "add" => {
                // '+' plus with small dropdown arrow
                let pc = Pos2::new(center.x - 4.0, center.y);
                painter.line_segment([Pos2::new(pc.x - 3.5, pc.y), Pos2::new(pc.x + 3.5, pc.y)], stroke);
                painter.line_segment([Pos2::new(pc.x, pc.y - 3.5), Pos2::new(pc.x, pc.y + 3.5)], stroke);
                let dc = Pos2::new(center.x + 5.0, center.y);
                painter.line_segment([Pos2::new(dc.x - 2.5, dc.y - 1.5), Pos2::new(dc.x, dc.y + 1.5)], stroke);
                painter.line_segment([Pos2::new(dc.x, dc.y + 1.5), Pos2::new(dc.x + 2.5, dc.y - 1.5)], stroke);
            }
            "more" => {
                let cy = center.y;
                painter.circle_filled(Pos2::new(center.x - 3.5, cy), 1.2, color);
                painter.circle_filled(Pos2::new(center.x, cy), 1.2, color);
                painter.circle_filled(Pos2::new(center.x + 3.5, cy), 1.2, color);
            }
            "maximize" => {
                let r = Rect::from_center_size(center, Vec2::new(10.0, 9.0));
                painter.rect_stroke(r, 1.0, stroke);
                painter.line_segment([Pos2::new(r.min.x + 2.0, r.min.y + 2.0), Pos2::new(r.max.x - 2.0, r.min.y + 2.0)], Stroke::new(1.0, color));
            }
            "close" => {
                let pad = 4.0;
                painter.line_segment([Pos2::new(rect.min.x + pad, rect.min.y + pad), Pos2::new(rect.max.x - pad, rect.max.y - pad)], stroke);
                painter.line_segment([Pos2::new(rect.max.x - pad, rect.min.y + pad), Pos2::new(rect.min.x + pad, rect.max.y - pad)], stroke);
            }
            _ => {}
        }

        response.clicked()
    }
}


