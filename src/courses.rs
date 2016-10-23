pub struct Courses {
    inner: Vec<String>,
}

impl Courses {
    pub fn new() -> Courses {
        Courses {
            inner: Vec::new(),
        }
    }

    pub fn add_course<I>(&mut self, course: I)
        where I: Into<String>
    {
        self.inner.push(course.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_courses_add_course() {
        let course1 = "Math";
        let course2 = "Literature".to_string();

        let mut courses = Courses::new();
        courses.add_course(course1);
        courses.add_course(course2);
    }
}
