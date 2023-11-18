use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modifier {
    Public,
    Friendly,
    Private,
    Protected
}

impl Default for Modifier {
    fn default() -> Self {
        Self::Friendly
    }
}

impl Modifier {
    pub fn is_public(&self) -> bool {
        *self == Self::Public
    }
    pub fn is_friendly(&self) -> bool {
        *self == Self::Friendly
    }
    pub fn is_private(&self) -> bool {
        *self == Self::Private
    }
    pub fn is_protected(&self) -> bool {
        *self == Self::Protected
    }
}

impl Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Modifier::Public => String::from("public"),
            Modifier::Friendly => String::new(),
            Modifier::Private => String::from("private"),
            Modifier::Protected => String::from("protected"),
        };
        f.write_str(s.to_string().as_str())
    }
}