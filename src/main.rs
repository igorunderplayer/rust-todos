use serde::{Serialize, Deserialize};

use std::{
    fs::{self, File},
    io::{stdin, Write},
    path::Path
};

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    complete_description: String,
    complete: bool
}

fn main() {
    println!("Bem vindo!");
    show_menu_message();

    let mut option = String::new();

    stdin().read_line(&mut option).expect("Burrao");

    println!("{}", option);

    if option.trim() == "1" {
        let todos = get_all_todos();
        println!("Total todos: {}", todos.len());
        for todo in todos {
            println!("[{}] {}", if todo.complete { "X" } else { " " }, todo.title);
        }
    }

    if option.trim() == "3" {
        let mut todo_name = String::new();
        let mut todo_description = String::new();

        println!("Me fala o nome da sua nova todo uhul");
        stdin().read_line(&mut todo_name).expect("Nome burrao");
        
        println!("Me uma desc agr");
        stdin().read_line(&mut todo_description).expect("Desc burrao");

        let new_todo = Todo {
            title: todo_name.replace("\n", ""),
            complete_description: todo_description.replace("\n", ""),
            complete: false
        };

        create_todo(new_todo);
    }
}

fn show_menu_message() {
    println!("Use uma das opções abaixo:");
    println!("1 | Ver todos os seus todos");
    println!("2 | Ver/editar uma todo");
}

fn get_all_todos() -> Vec<Todo> {
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

fn create_todo(data: Todo) {
    let path = Path::new(".todos.json");
    let mut all_todos = get_all_todos();
    all_todos.insert(all_todos.len() - 1, data);

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    let json = serde_json::to_string(&all_todos).unwrap();
    file.write_all(json.as_bytes()).expect("Error on writing data to file");
}
