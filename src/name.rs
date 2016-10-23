use std::ops::{Deref, DerefMut};
use std::mem;

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

    pub fn as_name_str(&self) -> &NameStr {
        unsafe { mem::transmute(& *self.inner) }
    }

    pub fn as_mut_name_str(&mut self) -> &mut NameStr {
        unsafe { mem::transmute(&mut *self.inner) }
    }

    pub fn into_string(self) -> String {
        self.inner
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        self.inner.as_mut_str()
    }

    pub fn uppercase(&mut self) {
        self.inner = self.inner.to_uppercase();
    }
}

impl AsRef<NameStr> for NameString {
    fn as_ref(&self) -> &NameStr {
        self.as_name_str()
    }
}

impl AsMut<NameStr> for NameString {
    fn as_mut(&mut self) -> &mut NameStr {
        self.as_mut_name_str()
    }
}

impl Deref for NameString {
    type Target = NameStr;

    fn deref(&self) -> &NameStr {
        unsafe { mem::transmute(& *self.inner) }
    }
}

impl DerefMut for NameString {
    fn deref_mut(&mut self) -> &mut NameStr {
        unsafe { mem::transmute(&mut *self.inner) }
    }
}

impl<'a> From<&'a NameStr> for NameString {
    fn from(n: &'a NameStr) -> Self {
        NameString {
            inner: n.inner.to_string()
        }
    }
}

impl From<String> for NameString {
    fn from(s: String) -> Self {
        NameString {
            inner: s
        }
    }
}

impl Into<String> for NameString {
    fn into(self) -> String {
        self.inner
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct NameStr {
    // allowed per https://doc.rust-lang.org/book/unsized-types.html
    inner: str,
}

impl NameStr {
    pub fn new(name: &str) -> &NameStr {
        unsafe { mem::transmute(name) }
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

    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

impl AsRef<NameStr> for NameStr {
    fn as_ref(&self) -> &NameStr {
        self
    }
}

impl AsMut<NameStr> for NameStr {
    fn as_mut(&mut self) -> &mut NameStr {
        self
    }
}

impl<'a> Into<String> for &'a NameStr {
    fn into(self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_string_push() {
        let expected = NameString { inner: "Given".to_string() };
        let mut given = NameString::new();
        given.push("Given");
        assert_eq!(expected, given);

        let expected = NameString { inner: "Given Family".to_string() };
        given.push(" Family");
        assert_eq!(expected, given);
    }

    #[test]
    fn test_name_string_uppercase() {
        let expected = NameString::from_str("NAME");
        let mut given = NameString:: from_str("Name");
        given.uppercase();
        assert_eq!(expected, given);
    }

    #[test]
    fn test_name_string_from_str() {
        let mut expected = NameString::new();
        expected.push("Name");
        let given = NameString::from_str("Name");
        assert_eq!(expected, given);
    }

    #[test]
    fn test_name_string_as_name_str() {
        let expected = NameStr::new("Name");
        let mut given = NameString::new();
        given.push("Name");
        assert_eq!(expected, given.as_name_str());
    }

    #[test]
    fn test_name_string_as_mut_name_str() {
        let mut name = NameString::from_str("Name");
        let _name_ref: &mut NameStr = name.as_mut_name_str();
    }

    #[test]
    fn test_name_string_into_string() {
        let expected = String::from("Name");
        let mut given = NameString::new();
        given.push("Name");
        assert_eq!(expected, given.into_string());
    }

    #[test]
    fn test_name_string_as_str() {
        let expected = "Name";
        let mut given = NameString::new();
        given.push("Name");
        assert_eq!(expected, given.as_str());
    }

    #[test]
    fn test_name_string_as_mut_str() {
        let mut name = "Name".to_string();
        let expected = name.as_mut_str();
        let mut given = NameString::from_str("Name");
        assert_eq!(expected, given.as_mut_str());
    }

    #[test]
    fn test_name_string_deref() {
        let given = NameString::from_str("Name");
        let expected = NameStr::new("Name");
        assert_eq!(expected, my_deref(&given));

        fn my_deref(n: &NameStr) -> &NameStr {
            n
        }
    }

    #[test]
    fn test_name_string_deref_mut() {
        let mut given = NameString::from_str("Name");
        let expected = NameStr::new("Name");
        assert_eq!(expected, my_deref_mut(&mut given));

        fn my_deref_mut(n: &mut NameStr) -> &mut NameStr {
            n
        }
    }

    #[test]
    fn test_name_string_as_ref() {
        let expected = NameStr::new("Name");
        let given = NameString::from_str("Name");
        assert_eq!(expected, given.as_ref());
    }

    #[test]
    fn test_name_string_as_mut() {
        let mut name = NameString::from_str("Name");
        let _name_ref: &mut NameStr = name.as_mut();
    }

    #[test]
    fn test_name_str_as_ref() {
        let expected = NameStr::new("Name");
        assert_eq!(expected, expected.as_ref());
    }

    #[test]
    fn test_name_str_as_mut() {
        let mut name = NameString::from_str("Name");
        let name_ref: &mut NameStr = name.as_mut();
        let _expected: &mut NameStr = name_ref.as_mut();
    }

    #[test]
    fn test_name_string_from_string() {
        let expected = NameString::from_str("Name");
        let given = String::from("Name");
        assert_eq!(expected, NameString::from(given));
    }

    #[test]
    fn string_into_name_string() {
        let expected = NameString::from_str("Name");
        let given = String::from("Name");
        assert_eq!(expected, given.into());
    }

    #[test]
    fn test_name_string_into() {
        let expected = String::from("Name");
        let name = NameString::from_str("Name");
        let given: String = name.into();
        assert_eq!(expected, given);
    }

    #[test]
    fn test_name_string_from_name_str() {
        let expected = NameString::from_str("Name");
        let given = NameStr::new("Name");
        assert_eq!(expected, NameString::from(given));
    }

    #[test]
    fn test_name_str_into_name_string() {
        let expected = NameString::from_str("Name");
        let given = NameStr::new("Name");
        assert_eq!(expected, given.into());
    }

    #[test]
    fn test_name_str_family() {
        let name = NameStr::new("");
        assert_eq!(None, name.family());

        let name = NameStr::new("Family");
        assert_eq!(Some("Family"), name.family());

        let name = NameStr::new("Given Family");
        assert_eq!(Some("Family"), name.family());
    }

    #[test]
    fn test_name_str_given() {
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
    fn test_name_str_to_name_string() {
        let name_str = NameStr::new("Given S. Family");
        let mut name_string = NameString::new();
        name_string.push("Given S. Family");
        assert_eq!(name_string, name_str.to_name_string());
    }

    #[test]
    fn test_name_str_to_string() {
        let name = NameStr::new("Name");
        let given = name.to_string();
        let expected = "Name".to_string();

        assert_eq!(expected, given);
    }

    #[test]
    fn test_name_str_into() {
        let name = NameStr::new("Name");
        let given: String = name.into();
        let expected = "Name".to_string();

        assert_eq!(expected, given);
    }
}
