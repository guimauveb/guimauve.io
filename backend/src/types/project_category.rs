use {
    diesel_derive_enum::DbEnum,
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, DbEnum, Eq, Hash)]
pub enum ProjectCategory {
    WebApplication,
    DesktopApplication,
}

impl Default for ProjectCategory {
    fn default() -> Self {
        Self::WebApplication
    }
}

impl Display for ProjectCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::WebApplication => write!(f, "Web applications"),
            Self::DesktopApplication => write!(f, "Desktop applications"),
        }
    }
}
