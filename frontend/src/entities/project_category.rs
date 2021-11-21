use {
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result},
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
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
            ProjectCategory::WebApplication => write!(f, "Web applications"),
            ProjectCategory::DesktopApplication => write!(f, "Desktop applications"),
        }
    }
}
