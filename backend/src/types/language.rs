use {
    diesel_derive_enum::DbEnum,
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, DbEnum, Hash)]
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
        Self::Bash
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{self:?}")
    }
}
