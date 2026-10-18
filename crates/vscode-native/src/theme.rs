use egui::{Color32, Stroke, Vec2};

#[allow(dead_code)]
pub const COLOR_ACTIVITY_BAR: Color32 = Color32::from_rgb(51, 51, 51);      // #333333
#[allow(dead_code)]
pub const COLOR_SIDEBAR_BG: Color32 = Color32::from_rgb(37, 37, 38);        // #252526
#[allow(dead_code)]
pub const COLOR_SIDEBAR_HEADER: Color32 = Color32::from_rgb(45, 45, 45);    // #2d2d2d
#[allow(dead_code)]
pub const COLOR_EDITOR_BG: Color32 = Color32::from_rgb(30, 30, 30);         // #1e1e1e
#[allow(dead_code)]
pub const COLOR_STATUS_BAR: Color32 = Color32::from_rgb(0, 122, 204);       // #007acc
#[allow(dead_code)]
pub const COLOR_TAB_ACTIVE: Color32 = Color32::from_rgb(30, 30, 30);        // #1e1e1e
#[allow(dead_code)]
pub const COLOR_TAB_INACTIVE: Color32 = Color32::from_rgb(45, 45, 45);      // #2d2d2d
#[allow(dead_code)]
pub const COLOR_TEXT_PRIMARY: Color32 = Color32::from_rgb(204, 204, 204);   // #cccccc
#[allow(dead_code)]
pub const COLOR_TEXT_MUTED: Color32 = Color32::from_rgb(140, 140, 140);     // #8c8c8c
#[allow(dead_code)]
pub const COLOR_BORDER: Color32 = Color32::from_rgb(60, 60, 60);           // #3c3c3c
#[allow(dead_code)]
pub const COLOR_LINE_NUMBERS: Color32 = Color32::from_rgb(133, 133, 133);   // #858585


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemePreset {
    Antigravity,
    DarkPlus,
    Monokai,
    SolarizedDark,
    GitHubDark,
    LightPlus,
}

#[derive(Clone, Debug)]
pub struct ThemeColors {
    pub preset: ThemePreset,
    pub name: &'static str,
    pub activity_bar: Color32,
    pub sidebar_bg: Color32,
    pub sidebar_header: Color32,
    pub editor_bg: Color32,
    pub status_bar: Color32,
    pub tab_active: Color32,
    pub tab_inactive: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub border: Color32,
    pub line_numbers: Color32,
    pub accent: Color32,
    pub input_bg: Color32,
    pub input_border: Color32,
    pub syntect_theme: &'static str,
    pub is_dark: bool,
}

impl ThemeColors {
    pub fn from_preset(preset: ThemePreset) -> Self {
        match preset {
            ThemePreset::Antigravity => Self {
                preset,
                name: "Native VS Dark (Default)",
                activity_bar: Color32::from_rgb(24, 24, 28),     // #18181c
                sidebar_bg: Color32::from_rgb(30, 30, 36),       // #1e1e24
                sidebar_header: Color32::from_rgb(33, 33, 40),   // #212128
                editor_bg: Color32::from_rgb(30, 30, 36),        // #1e1e24 distinct dark surface
                status_bar: Color32::from_rgb(0, 122, 204),       // #007acc iconic VS Code blue status bar
                tab_active: Color32::from_rgb(30, 30, 36),       // #1e1e24
                tab_inactive: Color32::from_rgb(24, 24, 28),     // #18181c
                text_primary: Color32::from_rgb(244, 244, 245),  // #f4f4f5
                text_muted: Color32::from_rgb(161, 161, 170),    // #a1a1aa
                border: Color32::from_rgb(48, 48, 56),           // #303038 clear clean borders
                line_numbers: Color32::from_rgb(110, 115, 130),  // #6e7382 legible line numbers
                accent: Color32::from_rgb(56, 189, 248),         // #38bdf8 Native sky blue
                input_bg: Color32::from_rgb(50, 50, 62),         // #32323e clearly visible elevated input surface
                input_border: Color32::from_rgb(74, 74, 92),     // #4a4a5c crisp clean border
                syntect_theme: "base16-ocean.dark",
                is_dark: true,
            },
            ThemePreset::DarkPlus => Self {
                preset,
                name: "Dark+ (VS Code Default)",
                activity_bar: Color32::from_rgb(51, 51, 51),
                sidebar_bg: Color32::from_rgb(37, 37, 38),
                sidebar_header: Color32::from_rgb(45, 45, 45),
                editor_bg: Color32::from_rgb(30, 30, 30),
                status_bar: Color32::from_rgb(0, 122, 204),
                tab_active: Color32::from_rgb(30, 30, 30),
                tab_inactive: Color32::from_rgb(45, 45, 45),
                text_primary: Color32::from_rgb(204, 204, 204),
                text_muted: Color32::from_rgb(140, 140, 140),
                border: Color32::from_rgb(60, 60, 60),
                line_numbers: Color32::from_rgb(133, 133, 133),
                accent: Color32::from_rgb(0, 122, 204),
                input_bg: Color32::from_rgb(60, 60, 60),
                input_border: Color32::from_rgb(75, 75, 75),
                syntect_theme: "base16-ocean.dark",
                is_dark: true,
            },
            ThemePreset::Monokai => Self {
                preset,
                name: "Monokai",
                activity_bar: Color32::from_rgb(31, 31, 31),
                sidebar_bg: Color32::from_rgb(37, 37, 37),
                sidebar_header: Color32::from_rgb(45, 45, 45),
                editor_bg: Color32::from_rgb(39, 40, 34),
                status_bar: Color32::from_rgb(117, 113, 94),
                tab_active: Color32::from_rgb(39, 40, 34),
                tab_inactive: Color32::from_rgb(30, 31, 28),
                text_primary: Color32::from_rgb(248, 248, 242),
                text_muted: Color32::from_rgb(117, 113, 94),
                border: Color32::from_rgb(50, 50, 50),
                line_numbers: Color32::from_rgb(144, 144, 138),
                accent: Color32::from_rgb(230, 219, 116),
                input_bg: Color32::from_rgb(48, 49, 44),
                input_border: Color32::from_rgb(65, 65, 60),
                syntect_theme: "base16-mocha.dark",
                is_dark: true,
            },
            ThemePreset::SolarizedDark => Self {
                preset,
                name: "Solarized Dark",
                activity_bar: Color32::from_rgb(0, 33, 43),
                sidebar_bg: Color32::from_rgb(7, 54, 66),
                sidebar_header: Color32::from_rgb(0, 43, 54),
                editor_bg: Color32::from_rgb(0, 43, 54),
                status_bar: Color32::from_rgb(38, 139, 210),
                tab_active: Color32::from_rgb(0, 43, 54),
                tab_inactive: Color32::from_rgb(7, 54, 66),
                text_primary: Color32::from_rgb(131, 148, 150),
                text_muted: Color32::from_rgb(88, 110, 117),
                border: Color32::from_rgb(10, 60, 75),
                line_numbers: Color32::from_rgb(88, 110, 117),
                accent: Color32::from_rgb(42, 161, 152),
                input_bg: Color32::from_rgb(10, 48, 59),
                input_border: Color32::from_rgb(18, 65, 80),
                syntect_theme: "Solarized (dark)",
                is_dark: true,
            },
            ThemePreset::GitHubDark => Self {
                preset,
                name: "GitHub Dark",
                activity_bar: Color32::from_rgb(1, 4, 9),
                sidebar_bg: Color32::from_rgb(13, 17, 23),
                sidebar_header: Color32::from_rgb(22, 27, 34),
                editor_bg: Color32::from_rgb(13, 17, 23),
                status_bar: Color32::from_rgb(31, 111, 235),
                tab_active: Color32::from_rgb(13, 17, 23),
                tab_inactive: Color32::from_rgb(22, 27, 34),
                text_primary: Color32::from_rgb(201, 209, 217),
                text_muted: Color32::from_rgb(139, 148, 158),
                border: Color32::from_rgb(48, 54, 61),
                line_numbers: Color32::from_rgb(110, 118, 129),
                accent: Color32::from_rgb(46, 160, 67),
                input_bg: Color32::from_rgb(33, 38, 45),
                input_border: Color32::from_rgb(48, 54, 61),
                syntect_theme: "base16-ocean.dark",
                is_dark: true,
            },
            ThemePreset::LightPlus => Self {
                preset,
                name: "Light+ (VS Code Default)",
                activity_bar: Color32::from_rgb(44, 44, 44),
                sidebar_bg: Color32::from_rgb(243, 243, 243),
                sidebar_header: Color32::from_rgb(230, 230, 230),
                editor_bg: Color32::from_rgb(255, 255, 255),
                status_bar: Color32::from_rgb(0, 122, 204),
                tab_active: Color32::from_rgb(255, 255, 255),
                tab_inactive: Color32::from_rgb(236, 236, 236),
                text_primary: Color32::from_rgb(51, 51, 51),
                text_muted: Color32::from_rgb(113, 113, 113),
                border: Color32::from_rgb(218, 218, 218),
                line_numbers: Color32::from_rgb(150, 150, 150),
                accent: Color32::from_rgb(0, 122, 204),
                input_bg: Color32::from_rgb(255, 255, 255),
                input_border: Color32::from_rgb(206, 206, 206),
                syntect_theme: "base16-ocean.light",
                is_dark: false,
            },
        }
    }
}

pub fn apply_theme(ctx: &egui::Context, theme: &ThemeColors) {
    let mut visuals = if theme.is_dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    visuals.panel_fill = theme.editor_bg;
    visuals.window_fill = theme.sidebar_bg;
    visuals.faint_bg_color = theme.sidebar_header;
    visuals.extreme_bg_color = theme.input_bg;
    visuals.widgets.noninteractive.bg_fill = theme.sidebar_bg;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, theme.text_primary);
    visuals.widgets.inactive.bg_fill = theme.input_bg;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, theme.text_muted);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 65, 75);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.active.bg_fill = theme.tab_active;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.window_stroke = Stroke::new(1.0, theme.border);
    visuals.window_rounding = 0.0.into();

    let mut style = (*ctx.style()).clone();
    style.visuals = visuals;
    style.spacing.item_spacing = Vec2::new(4.0, 4.0);
    style.spacing.window_margin = egui::Margin::same(0.0);
    style.spacing.scroll.bar_width = 14.0;
    style.spacing.scroll.bar_inner_margin = 1.0;
    style.spacing.scroll.handle_min_length = 24.0;
    ctx.set_style(style);
}

#[allow(dead_code)]
pub fn setup_vscode_visuals(ctx: &egui::Context) {
    let theme = ThemeColors::from_preset(ThemePreset::Antigravity);
    apply_theme(ctx, &theme);
}




