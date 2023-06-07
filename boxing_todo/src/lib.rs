mod err;
use err::{ParseErr, ReadErr};

pub use json::{parse, stringify};
pub use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32, //
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file = std::fs::read_to_string(path);
        
        match file {
            Ok(file) => {
                let json = parse(&file);
                
                match json {
                    Ok(json) => {
                        let title = json["title"].as_str().unwrap().to_string();
                        let mut tasks = Vec::new();
                        if json["tasks"].members().len() == 0 {
                            Err(Box::new(ParseErr::Empty))
                        } else {
                            for task in json["tasks"].members() {
                                let id = task["id"].as_u32().unwrap();
                                let description = task["description"].as_str().unwrap().to_string();
                                let level = task["level"].as_u32().unwrap();
                                tasks.push(Task { id, description, level });
                            }
                            Ok(TodoList { title, tasks })
                        }
                    }
                    Err(e) => Err(Box::new(ParseErr::Malformed(Box::new(e)))),
                }
            }
            Err(e) => Err(Box::new(ReadErr { child_err: Box::new(e) })),
        }
    }
}