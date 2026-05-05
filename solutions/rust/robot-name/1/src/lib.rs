use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory{
    used_names: Rc<RefCell<HashSet<String>>>,
}

pub struct Robot{
    name: String,
    used_names: Rc<RefCell<HashSet<String>>>,
}

impl RobotFactory {
    pub fn new() -> Self {
        RobotFactory { 
            used_names: Rc::new(RefCell::new(HashSet::new())) 
        }
    }

    pub fn new_robot<R: Rng>(&mut self, rng: &mut R) -> Robot {
        let name = loop {
            let first_letter = rng.random_range('A'..='Z');
            let second_letter = rng.random_range('A'..='Z');
            let number: u32 = rng.random_range(0..1000);
            let name = format!("{}{}{:03}", first_letter, second_letter, number);
            if !self.used_names.borrow().contains(&name) {
                self.used_names.borrow_mut().insert(name.clone());
                break name;
            }
        };
        Robot { 
            name,
            used_names: Rc::clone(&self.used_names),
        }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, rng: &mut R) {
        self.used_names.borrow_mut().remove(&self.name);
        let new_name = loop {
            let first_letter = rng.random_range('A'..='Z');
            let second_letter = rng.random_range('A'..='Z');
            let number: u32 = rng.random_range(0..1000);
            let name = format!("{}{}{:03}", first_letter, second_letter, number);
            if !self.used_names.borrow().contains(&name) {
                self.used_names.borrow_mut().insert(name.clone());
                break name;
            }
        };
        self.name = new_name;
    }
}
