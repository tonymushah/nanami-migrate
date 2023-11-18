use std::fmt::Display;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    BuiltIn(BuiltInType),
    BuiltInTypeBoxed(BuiltInTypeBoxed),
    Specific(String)
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::BuiltIn(d) => d.fmt(f),
            Type::BuiltInTypeBoxed(d) => d.fmt(f),
            Type::Specific(d) => d.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BuiltInType{
    Int,
    String,
    Boolean,
    Short,
    Float,
    Byte,
    Char,
    Double,
}

impl BuiltInType{
    pub fn is_int(&self) -> bool {
        *self == Self::Int
    }
    pub fn is_string(&self) -> bool {
        *self == Self::String
    }
    pub fn is_boolean(&self) -> bool {
        *self == Self::Boolean
    } 
    pub fn is_short(&self) -> bool {
        *self == Self::Short
    }
    pub fn is_float(&self) -> bool {
        *self == Self::Float
    }
    pub fn is_byte(&self) -> bool {
        *self == Self::Byte
    }
    pub fn is_char(&self) -> bool {
        *self == Self::Char
    }
    pub fn is_double(&self) -> bool {
        *self == Self::Double
    }
}

impl Display for BuiltInType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BuiltInType::Int => String::from("int"),
            BuiltInType::String => String::from("String"),
            BuiltInType::Boolean => String::from("boolean"),
            BuiltInType::Short => String::from("short"),
            BuiltInType::Float => String::from("float"),
            BuiltInType::Byte => String::from("byte"),
            BuiltInType::Char => String::from("char"),
            BuiltInType::Double => String::from("double"),
        };
        f.write_str(s.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BuiltInTypeBoxed{
    Integer,
    String,
    Boolean,
    Short,
    Float,
    Byte,
    Char,
    Double,
}

impl BuiltInTypeBoxed{
    pub fn is_integer(&self) -> bool {
        *self == Self::Integer
    }
    pub fn is_string(&self) -> bool {
        *self == Self::String
    }
    pub fn is_boolean(&self) -> bool {
        *self == Self::Boolean
    } 
    pub fn is_short(&self) -> bool {
        *self == Self::Short
    }
    pub fn is_float(&self) -> bool {
        *self == Self::Float
    }
    pub fn is_byte(&self) -> bool {
        *self == Self::Byte
    }
    pub fn is_char(&self) -> bool {
        *self == Self::Char
    }
    pub fn is_double(&self) -> bool {
        *self == Self::Double
    }
}

impl Display for BuiltInTypeBoxed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BuiltInTypeBoxed::Integer => String::from("Integer"),
            BuiltInTypeBoxed::String => BuiltInType::String.to_string(),
            BuiltInTypeBoxed::Boolean => String::from("Boolean"),
            BuiltInTypeBoxed::Short => String::from("Short"),
            BuiltInTypeBoxed::Float => String::from("Float"),
            BuiltInTypeBoxed::Byte => String::from("Byte"),
            BuiltInTypeBoxed::Char => String::from("Char"),
            BuiltInTypeBoxed::Double => String::from("Double"),
        };
        f.write_str(s.as_str())
    }
}

impl From<BuiltInType> for BuiltInTypeBoxed {
    fn from(value: BuiltInType) -> Self {
        match value {
            BuiltInType::Int => Self::Integer,
            BuiltInType::String => Self::String,
            BuiltInType::Boolean => Self::Boolean,
            BuiltInType::Short => Self::Short,
            BuiltInType::Float => Self::Float,
            BuiltInType::Byte => Self::Byte,
            BuiltInType::Char => Self::Char,
            BuiltInType::Double => Self::Double,
        }
    }
}

impl From<BuiltInTypeBoxed> for BuiltInType{
    fn from(value: BuiltInTypeBoxed) -> Self {
        match value {
            BuiltInTypeBoxed::Integer => Self::Int,
            BuiltInTypeBoxed::String => Self::String,
            BuiltInTypeBoxed::Boolean => Self::Boolean,
            BuiltInTypeBoxed::Short => Self::Short,
            BuiltInTypeBoxed::Float => Self::Float,
            BuiltInTypeBoxed::Byte => Self::Byte,
            BuiltInTypeBoxed::Char => Self::Char,
            BuiltInTypeBoxed::Double => Self::Double,
        }
    }
}