use boxing_todo::TodoList;

fn main() {
    let todos = TodoList::get_todo("todo.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}{:?}", e.to_string(), e.source());
        }
    }
    let todos = TodoList::get_todo("todo_empty.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}{:?}", e.to_string(), e.source());
        }
    }
    let todos = TodoList::get_todo("malformed_object.json");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}{:?}", e.to_string(), e.source());
        }
    }
}