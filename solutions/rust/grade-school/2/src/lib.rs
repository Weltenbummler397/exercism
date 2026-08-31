use std::collections::HashMap;
pub struct School {
    classes: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School{classes: HashMap::new(),}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let student_exists_anywhere = self.classes.values().any(|list| list.contains(&student.to_string()));
        if !student_exists_anywhere {
            self.classes
                .entry(grade)
                .or_default()
                .push(student.to_string());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
         let mut all_grades: Vec<u32> = self.classes.keys().copied().collect();
        all_grades.sort();
        all_grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(students) = self.classes.get(&grade) {
        let mut sorted_students = students.clone();
        sorted_students.sort();
        sorted_students
    } else {
        Vec::new()
    }
    }
}
