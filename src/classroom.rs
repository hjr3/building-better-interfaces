use name::NameString;
use roster::Roster;

#[derive(Debug, Eq, PartialEq)]
pub struct Classroom {
    names: Vec<NameString>,
}

impl Classroom {
    pub fn new() -> Classroom {
        Classroom {
            names: Vec::new(),
        }
    }

    pub fn with_names(names: Vec<NameString>) -> Classroom {
        Classroom {
            names: names
        }
    }

    pub fn add_name<I>(&mut self, name: I)
        where I: Into<NameString>
    {
        self.names.push(name.into())
    }

    pub fn as_roster(&self) -> Roster {
        let mut r = Roster::new();

        for name in self.names.iter() {
            r.add_name(name);
        }

        r
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use name::{NameStr, NameString};
    use roster::Roster;

    #[test]
    fn test_classroom_new() {
        let _ = Classroom::new();
    }

    #[test]
    fn test_classroom_add_name() {
        let name = NameString::from_str("Name");

        let mut c = Classroom::new();
        c.add_name(name);
    }

    #[test]
    fn test_classroom_add_name_using_into() {
        let mut c = Classroom::new();
        c.add_name(NameStr::new("Name"));
    }

    #[test]
    fn test_classroom_with_names() {
        let names = vec![
            NameString::from_str("Name1"),
            NameString::from_str("Name2"),
        ];

        let given = Classroom::with_names(names);

        let mut expected = Classroom::new();
        expected.add_name(NameString::from_str("Name1"));
        expected.add_name(NameString::from_str("Name2"));

        assert_eq!(expected, given);
    }

    #[test]
    fn test_classroom_as_roster() {
        let names = vec![
            NameString::from_str("Name1"),
            NameString::from_str("Name2"),
        ];

        let c = Classroom::with_names(names);
        let given = c.as_roster();

        let mut expected = Roster::new();
        expected.add_name(NameStr::new("Name1"));
        expected.add_name(NameStr::new("Name2"));

        assert_eq!(expected, given);
    }
}
