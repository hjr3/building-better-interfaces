use std::ops::Deref;
use name::NameString;

pub struct Student {
    name: NameString,
}

impl Student {
    pub fn name(&self) -> &NameString {
        &self.name
    }
}

impl Deref for Student {
    type Target = NameString;

    fn deref(&self) -> &NameString {
        &self.name
    }
}

pub struct InClass<T> {
    inner: T,
}

impl<T> InClass<T> {
    pub fn play(self) -> Recess<T> {
        Recess {
            inner: self.inner,
        }
    }
}

impl<T> Deref for InClass<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

pub struct Recess<T> {
    inner: T,
}

impl<T> Recess<T> {
    pub fn learn(self) -> InClass<T> {
        InClass {
            inner: self.inner
        }
    }

    pub fn shout(&self) -> &str {
        "WOO!"
    }
}

impl<T> Deref for Recess<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use student::{Student, InClass, Recess};
    use name::NameString;

    #[test]
    fn test_student_state_changes() {

        let me = Student { name: NameString::from_str("Herman") };
        let me_in_class = InClass { inner: me };
        let me_at_recess = me_in_class.play();
        let _me_back_in_class = me_at_recess.learn();

        let me = Student { name: NameString::from_str("Herman") };
        let me_at_recess = Recess { inner: me };
        assert_eq!("WOO!", me_at_recess.shout());
        let _me_in_class = me_at_recess.learn();
    }

    #[test]
    fn test_student_deref_name_str() {
        let me = Student { name: NameString::from_str("Herman Radtke") };
        assert_eq!(Some("Herman"), me.given());
    }

    #[test]
    fn test_in_class_deref_name_str() {
        let me = Student { name: NameString::from_str("Herman Radtke") };
        let me_in_class = InClass { inner: me };
        assert_eq!(Some("Herman"), me_in_class.given());
    }
}
