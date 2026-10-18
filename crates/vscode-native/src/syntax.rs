use eframe::egui::Color32;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    pub theme_name: String,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            theme_name: "base16-ocean.dark".to_string(),
        }
    }

    pub fn set_theme(&mut self, name: &str) {
        if self.theme_set.themes.contains_key(name) {
            self.theme_name = name.to_string();
        }
    }

    pub fn highlight_line<'a>(&'a self, line: &'a str, extension: &str) -> Vec<(Color32, &'a str)> {
        let syntax = self
            .syntax_set
            .find_syntax_by_extension(extension)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = self
            .theme_set
            .themes
            .get(&self.theme_name)
            .or_else(|| self.theme_set.themes.get("base16-ocean.dark"))
            .unwrap();
        let mut highlighter = HighlightLines::new(syntax, theme);

        match highlighter.highlight_line(line, &self.syntax_set) {
            Ok(ranges) => ranges
                .into_iter()
                .map(|(style, text)| {
                    let c = Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                    (c, text)
                })
                .collect(),
            Err(_) => vec![(Color32::from_rgb(204, 204, 204), line)],
        }
    }
}

