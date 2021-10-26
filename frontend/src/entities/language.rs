use {
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    Bash,
    Python,
    Sql,
    Html,
    Css,
    Javascript,
    Typescript,
    Yaml,
}

impl Default for Language {
    fn default() -> Self {
        Language::Bash
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

pub const LANGUAGES: &[Language] = &[
    Language::Rust,
    Language::Bash,
    Language::Python,
    Language::Sql,
    Language::Html,
    Language::Css,
    Language::Javascript,
    Language::Typescript,
    Language::Yaml,
];
