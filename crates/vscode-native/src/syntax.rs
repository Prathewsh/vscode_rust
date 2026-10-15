use eframe::egui::Color32;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight_line<'a>(&'a self, line: &'a str, extension: &str) -> Vec<(Color32, &'a str)> {
        let syntax = self
            .syntax_set
            .find_syntax_by_extension(extension)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];
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
