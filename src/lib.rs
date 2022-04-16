use serde::{Serialize, Deserialize};

use std::{
    fs::{self, File},
    io::Write,
    path::Path
};


#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub complete_description: String,
    pub complete: bool
}

pub fn get_all_todos() -> Vec<Todo> {
    let path = Path::new(".todos.json");
    if !path.exists() {
        let base_data = "[]";
        let mut file = File::create(path).expect("Error on creating file");
        write!(file, "{}", base_data).expect("Error on writing file");
    }
    let content = fs::read_to_string(path).expect("AAA");
    let todos: Vec<Todo> = serde_json::from_str(&content).expect("Error on deserealize json");
    return todos;
}

pub fn create_todo(data: Todo) {
    let mut all_todos = get_all_todos();
    all_todos.insert(all_todos.len(), data);
    update_todos_file(&all_todos);
}

pub fn delete_todo(index: usize) {
    let mut all_todos = get_all_todos();
    all_todos.remove(index);
    update_todos_file(&all_todos);
}

pub fn update_todos_file(todos: &Vec<Todo>) {
    let path = Path::new(".todos.json");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let json = serde_json::to_string(todos).unwrap();
    file.write_all(json.as_bytes()).expect("Error on writing data to file");
}
