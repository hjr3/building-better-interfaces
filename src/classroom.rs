use student::Student;

#[derive(Debug, Eq, PartialEq)]
pub struct Classroom {
    seats: Vec<Option<Student>>,
}

impl Classroom {
    pub fn new() -> Classroom {
        Classroom::with_seats(32)
    }

    pub fn with_seats(seat_cnt: usize) -> Classroom {

        let seats = (0..seat_cnt)
            .map(|_| None)
            .collect::<Vec<_>>();

        Classroom {
            seats: seats,
        }
    }

    pub fn add_student<F>(&mut self, f: F) -> Result<usize, &'static str>
        where F: FnOnce(usize) -> Student
    {
        match self.seats.iter().position(|seat| seat.is_none()) {
            Some(seat) => {
                let student = f(seat);
                self.seats.push(Some(student));
                Ok(seat)
            }
            None => {
                Err("No more seats available")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use name::NameString;
    use student::Student;

    #[test]
    fn test_classroom_add_student() {

        let mut c = Classroom::new();
        let seat = c.add_student(|seat| {
            Student::new(NameString::from_str("Name"), seat)
        });

        assert_eq!(Ok(0), seat);
    }

    #[test]
    fn test_classroom_add_student_fails() {

        let mut c = Classroom::with_seats(0);
        let seat = c.add_student(|seat| {
            Student::new(NameString::from_str("Name"), seat)
        });

        assert_eq!(Err("No more seats available"), seat);
    }
}
