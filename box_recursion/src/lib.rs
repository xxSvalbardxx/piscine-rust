/* #[derive(Debug)]
pub struct WorkEnvironment { // WorkEnvironment is a struct 
    pub grade: Link, // Link is a type alias for Option<Box<Worker>>
}

pub type Link = Option<Box<Worker>>; 

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Box::new(Worker {
            role: role,
            name: name,
            next: self.grade.take(), // take() is a method of Option<T> that replaces the value of the Option<T> with None and returns the original value
        });
        self.grade = Some(new_worker);
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|worker| { // 
            self.grade = worker.next;
            worker.name
        })
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|worker| { // as_ref() is a method of Option<T> that returns an Option<&T>
            (worker.role.clone(), worker.name.clone())
        })
    }
} */

#[derive(Debug)]
pub struct WorkEnvironment { // WorkEnvironment is a struct 
    pub grade: Link, // Link is a type alias for Option<Box<Worker>>
}

pub type Link = Option<Box<Worker>>; 

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Box::new(Worker {
            role: role,
            name: name,
            next: self.grade.take(), // take() is a method of Option<T> that replaces the value of the Option<T> with None and returns the original value
        });
        self.grade = Some(new_worker);
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|worker| { 
            self.grade = worker.next;
            worker.name
        })
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|worker| { // as_ref() is a method of Option<T> that returns an Option<&T>
                (worker.name.clone(), worker.role.clone())
        })
    }
}