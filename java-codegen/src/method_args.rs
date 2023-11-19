use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::Type;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MethodArg {
    name: String,
    type_: Type,
}

impl MethodArg {
    pub fn new(name: String, type_: Type) -> Self {
        Self { name, type_ }
    }
}

impl Display for MethodArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.type_, self.name))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MethodArgs(Vec<MethodArg>);

impl Deref for MethodArgs {
    type Target = Vec<MethodArg>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MethodArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for MethodArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.is_empty() {
            let end = self.len() - 1;
            for (index, arg) in self.iter().enumerate() {
                arg.fmt(f)?;
                if index < end {
                    f.write_str(", ")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::{MethodArg, MethodArgs};
    #[test]
    fn method_arg_display(){
        let arg = MethodArg::new(String::from("tony"), crate::Type::BuiltIn(crate::BuiltInType::String));
        let res = format!("{}", arg);
        assert_eq!(res, String::from("String tony"));
    }
    #[test]
    fn method_args_display() {
        let mut args = MethodArgs::default();
        args.push(MethodArg::new(String::from("name"), crate::Type::BuiltIn(crate::BuiltInType::String)));
        args.push(MethodArg::new(String::from("birthday"), crate::Type::Specific(String::from("java.sql.DateTime"))));
        let res = format!("{args}");
        assert_eq!(res, String::from("String name, java.sql.DateTime birthday"))
    }
}