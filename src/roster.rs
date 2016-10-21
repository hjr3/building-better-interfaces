use name::NameStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Roster<'a> {
    pub names: Vec<&'a NameStr>,
}

impl<'a> Roster<'a> {
    pub fn new() -> Roster<'a> {
        Roster {
            names: Vec::new(),
        }
    }

    pub fn add_name(&mut self, name: &'a NameStr) {
        self.names.push(name)
    }
}

#[cfg(test)]
mod test {
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
}

