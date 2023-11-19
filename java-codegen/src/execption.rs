use std::{fmt::Display, ops::{Deref, DerefMut}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Exception(String);

impl Default for Exception {
    fn default() -> Self {
        Self(String::from("Exception"))
    }
}

impl Exception {
    pub fn new(name: String) -> Self{
        name.into()
    }
}

impl From<String> for Exception {
    fn from(value: String) -> Self {
        Exception(value)
    }
}

impl From<Exception> for String {
    fn from(value: Exception) -> Self {
        value.0
    }
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Exceptions(Vec<Exception>);

impl Deref for Exceptions {
    type Target = Vec<Exception>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Exceptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Exceptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.is_empty() {
            let end = self.len() - 1;
            for (index, exception) in self.iter().enumerate() {
                exception.fmt(f)?;
                if index < end {
                    f.write_str(", ")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Exception, Exceptions};

    #[test]
    fn test_exception_display() {
        let test = Exception::new(String::from("StackOverflowException"));
        let res = format!("{test}");
        assert_eq!(res, String::from("StackOverflowException"));
    }
    #[test]
    fn test_exceptions_display() {
        let mut test = Exceptions::default();
        test.push(String::from("TestException").into());
        test.push(String::from("StackOverflowException").into());
        let res = format!("{test}");
        assert_eq!(res, String::from("TestException, StackOverflowException"));
    }
}