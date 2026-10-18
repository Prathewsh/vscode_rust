use eframe::egui::{text::LayoutJob, Color32, FontId, TextFormat};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

#[derive(Clone)]
struct CachedLayout {
    text_hash: u64,
    text_len: usize,
    extension: String,
    theme_name: String,
    font_size_bits: u32,
    job: LayoutJob,
}

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    pub theme_name: String,
    cache: Mutex<Option<CachedLayout>>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            theme_name: "base16-ocean.dark".to_string(),
            cache: Mutex::new(None),
        }
    }

    pub fn set_theme(&mut self, name: &str) {
        if self.theme_set.themes.contains_key(name) {
            self.theme_name = name.to_string();
            if let Ok(mut guard) = self.cache.lock() {
                *guard = None;
            }
        }
    }

    pub fn highlight_text(&self, text: &str, extension: &str, font_id: FontId) -> LayoutJob {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let text_hash = hasher.finish();
        let font_size_bits = font_id.size.to_bits();

        if let Ok(guard) = self.cache.lock() {
            if let Some(ref cached) = *guard {
                if cached.text_hash == text_hash
                    && cached.text_len == text.len()
                    && cached.extension == extension
                    && cached.theme_name == self.theme_name
                    && cached.font_size_bits == font_size_bits
                {
                    return cached.job.clone();
                }
            }
        }

        let mut job = LayoutJob::default();
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

        for line in text.split_inclusive('\n') {
            if let Ok(ranges) = highlighter.highlight_line(line, &self.syntax_set) {
                for (style, token_str) in ranges {
                    let color = Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                    job.append(
                        token_str,
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color,
                            ..Default::default()
                        },
                    );
                }
            } else {
                job.append(
                    line,
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: Color32::from_rgb(204, 204, 204),
                        ..Default::default()
                    },
                );
            }
        }

        if let Ok(mut guard) = self.cache.lock() {
            *guard = Some(CachedLayout {
                text_hash,
                text_len: text.len(),
                extension: extension.to_string(),
                theme_name: self.theme_name.clone(),
                font_size_bits,
                job: job.clone(),
            });
        }

        job
    }

    #[allow(dead_code)]
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

