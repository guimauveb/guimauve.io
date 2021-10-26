#[cfg(feature = "editable")]
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

#[cfg(feature = "editable")]
pub fn highlight_code(code: &str, language: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let theme = &theme_set.themes["base16-ocean.dark"];
    let syntax = syntax_set
        .find_syntax_by_token(&language.to_lowercase())
        .unwrap();

    highlighted_html_for_string(code, &syntax_set, syntax, theme)
}
