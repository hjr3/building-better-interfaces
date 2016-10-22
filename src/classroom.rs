use name::NameString;

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
}

#[cfg(test)]
mod test {
    use super::*;
    use {NameStr, NameString};
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
}
