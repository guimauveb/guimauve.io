use {
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter, Result},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

pub const CONTENT_TYPES: &[ContentType] = &[
    ContentType::Text,
    ContentType::Comment,
    ContentType::Code,
    ContentType::Image,
    ContentType::Link,
];
