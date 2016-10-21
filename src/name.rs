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
    fn test_name_string_from_str() {
        let mut expected = NameString::new();
        expected.push("Name");
        let given = NameString::from_str("Name");
        assert_eq!(expected, given);
    }
}
