use name::NameStr;
use std::borrow::Cow;

#[derive(Debug, Eq, PartialEq)]
pub struct Roster<'a> {
    pub names: Vec<Cow<'a, NameStr>>,
}

impl<'a> Roster<'a> {
    pub fn new() -> Roster<'a> {
        Roster {
            names: Vec::new(),
        }
    }

    pub fn with_names(names: Vec<&'a NameStr>) -> Roster<'a> {
        let mut roster = Roster::new();
        for name in names {
            roster.add_name(name);
        }

        roster
    }

    pub fn add_name(&mut self, name: &'a NameStr) {
        self.names.push(name.to_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use name::{NameStr, NameString};

    #[test]
    fn test_roster_add_name() {
        let b_name = NameStr::new("Name");
        let o_name = NameString::from_str("Name");

        let mut o_name2 = NameString::from_str("Name");
        let name_mut = o_name2.as_mut();

        let mut r = Roster::new();
        r.add_name(b_name);
        r.add_name(&o_name);
        r.add_name(o_name.as_name_str());
        r.add_name(o_name.as_ref());
        r.add_name(name_mut);
    }

    #[test]
    fn test_roster_with_names() {
        let owned_name = NameString::from_str("Ada Lovelace");
        let names = vec![NameStr::new("Grace Hopper"), &owned_name];

        let given = Roster::with_names(names);

        let mut expected = Roster::new();
        expected.add_name(NameStr::new("Grace Hopper"));
        expected.add_name(NameStr::new("Ada Lovelace"));

        assert_eq!(expected, given);
    }
}

