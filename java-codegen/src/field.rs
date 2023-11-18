use std::fmt::Display;

use crate::{r#type::Type, Modifier};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field {
    modifier: Modifier,
    type_: Type,
    name: String,
    is_final: bool,
    is_static: bool,
    additional_statement: Option<String>,
}

impl Field {
    fn modifier(&self) -> String {
        if self.modifier.is_friendly() {
            String::new()
        } else {
            let mut m = self.modifier.to_string();
            m.push(' ');
            m
        }
    }
    fn additional_statement(&self) -> String {
        if let Some(additional_statement) = &self.additional_statement {
            format!(" = {}", additional_statement)
        } else {
            String::new()
        }
    }
    fn display_static_final(&self) -> String {
        format!(
            "{}static final {} {}{};",
            self.modifier(),
            self.type_,
            self.name,
            self.additional_statement()
        )
    }
    fn display_static(&self) -> String {
        format!(
            "{}static {} {}{};",
            self.modifier(),
            self.type_,
            self.name,
            self.additional_statement()
        )
    }
    fn display_final(&self) -> String {
        format!(
            "{}final {} {}{};",
            self.modifier(),
            self.type_,
            self.name,
            self.additional_statement()
        )
    }
    fn display_default(&self) -> String {
        format!(
            "{}{} {}{};",
            self.modifier(),
            self.type_,
            self.name,
            self.additional_statement()
        )
    }
    fn display(&self) -> String {
        if self.is_final && self.is_static {
            self.display_static_final()
        } else if !self.is_final && self.is_static {
            self.display_static()
        } else if self.is_final && !self.is_static {
            self.display_final()
        } else {
            self.display_default()
        }
    }
    pub fn new(name: String, type_: Type) -> Self {
        Self {
            modifier: Default::default(),
            type_,
            name,
            is_final: false,
            is_static: false,
            additional_statement: None,
        }
    }
    pub fn new_with_statement(name: String, type_: Type, statement: String) -> Self {
        Self {
            modifier: Default::default(),
            type_,
            name,
            is_final: false,
            is_static: false,
            additional_statement: Some(statement),
        }
    }
    pub fn set_modifier(&mut self, modifier: Modifier) -> &mut Self {
        self.modifier = modifier;
        self
    }
    pub fn set_type(&mut self, type_: Type) -> &mut Self {
        self.type_ = type_;
        self
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn set_final(&mut self, is_final: bool) -> &mut Self {
        self.is_final = is_final;
        self
    }
    pub fn set_static(&mut self, is_static: bool) -> &mut Self {
        self.is_static = is_static;
        self
    }
    pub fn set_statement(&mut self, statement: String) -> &mut Self {
        if !statement.is_empty() {
            let _ = self.additional_statement.insert(statement);
        }
        self
    }

    pub fn get_modifier(&self) -> &Modifier {
        &self.modifier
    }
    pub fn get_type(&self) -> &Type {
        &self.type_
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn is_final(&self) -> bool {
        self.is_final
    }
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    pub fn get_statement(&self) -> Option<&String> {
        self.additional_statement.as_ref()
    }
    pub fn clear_statement(&mut self) -> Option<String> {
        self.additional_statement.take()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display())
    }
}

#[cfg(test)]
mod tests {
    use super::Field;
    #[test]
    fn test_field() {
        let mut test = Field::new(
            String::from("tony"),
            crate::r#type::Type::BuiltIn(crate::r#type::BuiltInType::Int),
        );
        test.set_final(false);
        test.set_modifier(crate::Modifier::Protected);
        test.set_statement(String::from("Math.floor()"));
        println!("{test}",)
    }
}
