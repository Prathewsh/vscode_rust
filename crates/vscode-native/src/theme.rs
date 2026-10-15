use egui::{Color32, Stroke, Vec2};

pub const COLOR_ACTIVITY_BAR: Color32 = Color32::from_rgb(51, 51, 51);      // #333333
pub const COLOR_SIDEBAR_BG: Color32 = Color32::from_rgb(37, 37, 38);        // #252526
pub const COLOR_SIDEBAR_HEADER: Color32 = Color32::from_rgb(45, 45, 45);    // #2d2d2d
pub const COLOR_EDITOR_BG: Color32 = Color32::from_rgb(30, 30, 30);         // #1e1e1e
pub const COLOR_STATUS_BAR: Color32 = Color32::from_rgb(0, 122, 204);       // #007acc
pub const COLOR_TAB_ACTIVE: Color32 = Color32::from_rgb(30, 30, 30);        // #1e1e1e
pub const COLOR_TAB_INACTIVE: Color32 = Color32::from_rgb(45, 45, 45);      // #2d2d2d
pub const COLOR_TEXT_PRIMARY: Color32 = Color32::from_rgb(204, 204, 204);   // #cccccc
pub const COLOR_TEXT_MUTED: Color32 = Color32::from_rgb(140, 140, 140);     // #8c8c8c
pub const COLOR_BORDER: Color32 = Color32::from_rgb(60, 60, 60);           // #3c3c3c
pub const COLOR_LINE_NUMBERS: Color32 = Color32::from_rgb(133, 133, 133);   // #858585

pub fn setup_vscode_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = COLOR_EDITOR_BG;
    visuals.window_fill = COLOR_SIDEBAR_BG;
    visuals.faint_bg_color = COLOR_SIDEBAR_HEADER;
    visuals.extreme_bg_color = COLOR_EDITOR_BG;
    visuals.widgets.noninteractive.bg_fill = COLOR_SIDEBAR_BG;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, COLOR_TEXT_PRIMARY);
    visuals.widgets.inactive.bg_fill = COLOR_TAB_INACTIVE;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, COLOR_TEXT_MUTED);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(58, 61, 65);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.active.bg_fill = COLOR_TAB_ACTIVE;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.window_stroke = Stroke::new(1.0, COLOR_BORDER);
    visuals.window_rounding = 0.0.into();

    let mut style = (*ctx.style()).clone();
    style.visuals = visuals;
    style.spacing.item_spacing = Vec2::new(4.0, 4.0);
    style.spacing.window_margin = egui::Margin::same(0.0);
    ctx.set_style(style);
}

