// or do a Name thing with a Classroom where we make Report Cards

use std::ops::Deref;

#[derive(Eq, PartialEq, Debug)]
pub struct NameString {
    inner: String,
}

impl NameString {
    pub fn new() -> NameString {
        NameString {
            inner: String::new()
        }
    }

    pub fn from_str(s: &str) -> NameString {
        NameString {
            inner: s.to_string()
        }
    }

    pub fn push(&mut self, part: &str) {
        self.inner.push_str(part)
    }

    // We can later use Deref to make this work automatically
    pub fn as_name_str(&self) -> &NameStr {
        NameStr::new(self.inner.as_str())
    }

    pub fn into_string(self) -> String {
        self.inner
    }

    // Note: This uses Deref to work
    pub fn as_ref(&self) -> &NameStr {
        self
    }
}

impl Deref for NameString {
    type Target = NameStr;

    fn deref(&self) -> &NameStr {
        NameStr::new(self.inner.as_str())
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct NameStr {
    inner: str,
}

impl NameStr {
    pub fn new(name: &str) -> &NameStr {
        unsafe { std::mem::transmute(name) }
    }

    pub fn family(&self) -> Option<&str> {
        let iter = self.inner.split_whitespace();
        iter.last()
    }

    pub fn given(&self) -> Option<&str> {
        let mut iter = self.inner.split_whitespace();

        let g = iter.next();

        match iter.next() {
            Some(_) => g,
            None => None
        }
    }

    pub fn to_name_string(&self) -> NameString {
        NameString {
            inner: self.inner.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_string_push() {
        let expected = NameString { inner: "Name".to_string() };
        let mut given = NameString::new();
        given.push("Name");
        assert_eq!(expected, given);
    }

    #[test]
    fn name_string_from_str() {
        let mut expected = NameString::new();
        expected.push("Name");
        let given = NameString::from_str("Name");
        assert_eq!(expected, given);
    }

    #[test]
    fn name_string_as_name_str() {
        let expected = NameStr::new("Name");
        let mut given = NameString::new();
        given.push("Name");
        assert_eq!(expected, given.as_name_str());
    }

    #[test]
    fn name_string_into_string() {
        let expected = String::from("Name");
        let mut given = NameString::new();
        given.push("Name");
        assert_eq!(expected, given.into_string());
    }

    #[test]
    fn name_string_deref() {
        let given = NameString::from_str("Name");
        let expected = NameStr::new("Name");
        assert_eq!(expected, my_deref(&given));

        fn my_deref(n: &NameStr) -> &NameStr {
            n
        }
    }

    #[test]
    fn name_string_as_ref() {
        let given = NameString::from_str("Name");
        let expected = NameStr::new("Name");
        assert_eq!(expected, given.as_ref());
    }

    #[test]
    fn name_str_family() {
        let name = NameStr::new("");
        assert_eq!(None, name.family());

        let name = NameStr::new("Family");
        assert_eq!(Some("Family"), name.family());

        let name = NameStr::new("Given Family");
        assert_eq!(Some("Family"), name.family());
    }

    #[test]
    fn name_str_given() {
        let name = NameStr::new("");
        assert_eq!(None, name.given());

        let name = NameStr::new("Family");
        assert_eq!(None, name.given());

        let name = NameStr::new("Given Family");
        assert_eq!(Some("Given"), name.given());

        let name = NameStr::new("Given S. Family");
        assert_eq!(Some("Given"), name.given());
    }

    #[test]
    fn name_str_to_name_string() {
        let name_str = NameStr::new("Given S. Family");
        let mut name_string = NameString::new();
        name_string.push("Given S. Family");
        assert_eq!(name_string, name_str.to_name_string());
    }
}