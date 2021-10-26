use {
    diesel_derive_enum::DbEnum,
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, DbEnum, Hash)]
pub enum ContentType {
    Text,
    Comment,
    Link,
    Code,
    Image,
}

impl Default for ContentType {
    fn default() -> Self {
        ContentType::Text
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}
