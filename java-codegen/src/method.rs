use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::{method_args::MethodArgs, Modifier, Type, Exceptions};



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Method {
    pub modifier: Modifier,
    pub return_type: Type,
    pub args: MethodArgs,
    pub name: String,
    pub exceptions: Exceptions,
    pub body: String,
}

impl Method {
    fn modifier(&self) -> String {
        if !self.modifier.is_friendly() {
            format!("{} ", self.modifier)
        }else{
            String::new()
        }
    }
    fn return_type(&self) -> String {
        format!("{} ", self.return_type)
    }
    fn name(&self) -> String {
        format!("{} ", self.name)
    }
    fn head(&self) -> String {
        format!("{}{}{}({}) throws {} ", self.modifier(), self.return_type(), self.name(), self.args, self.exceptions)
    }
    fn body(&self) -> String {
        format!(r#"{}{}{}"#, "{", self.body, "}")
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.head())?;
        f.write_str(&self.body())
    }
}