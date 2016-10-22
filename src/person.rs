use name::NameString;

#[derive(Debug, Eq, PartialEq)]
pub struct Person {
    pub name: NameString,
}

impl Person {
    pub fn new<I>(n: I) -> Person
        where I: Into<NameString>
    {
        Person {
            name: n.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use name::{NameStr, NameString};

    #[test]
    fn test_person_new() {
        let o_name = NameString::from_str("Name");
        let p1 = Person::new(o_name);

        let b_name = NameStr::new("Name");
        let p2 = Person::new(b_name);

        assert_eq!(p1, p2);
    }
}

