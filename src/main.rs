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

    if option.trim() == "2" {
        let ref mut all_todos = get_all_todos();
        let mut i = 0;
        for todo in all_todos.iter() {
            i += 1;
            println!("{} - {}", i, todo.title)
        }
        println!("Digite o numero da todo que vc deseja modificar");
        let mut todo_index_string = String::new();
        stdin().read_line(&mut todo_index_string).expect("So coloca um numero");
        let todo_index = todo_index_string.trim().parse::<u32>().expect("o numero precisa ser maior q 0") - 1;
        if todo_index >= (all_todos.len() as u32) { return };

        let todo = all_todos.get(todo_index as usize).unwrap();
        let mut todo_option = String::new();
        println!("Informações da todo: {}", todo.title);
        println!("Descrição: {}", todo.complete_description);
        println!("Completada: {}", if todo.complete { "Sim" } else { "Não" });
        println!("=============");
        println!("(1) Completar/Descompletar? | (2) Apagar");
        stdin().read_line(&mut todo_option).unwrap();

        println!("opcao: {}", option);

        if todo_option.trim() == "1" {
            let new_todo = Todo {
                title: todo.title.to_string(),
                complete_description: todo.complete_description.to_string(),
                complete: !todo.complete
            };
            all_todos.remove(todo_index as usize);
            all_todos.insert(todo_index as usize, new_todo);
            update_todos_file(&all_todos);
        } else if option == "2" {}
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
    println!("3 | Criar uma nova todo");
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
    let mut all_todos = get_all_todos();
    all_todos.insert(all_todos.len() - 1, data);
    update_todos_file(&all_todos);
}

fn update_todos_file(todos: &Vec<Todo>) {
    let path = Path::new(".todos.json");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    let json = serde_json::to_string(&todos).unwrap();
    file.write_all(json.as_bytes()).expect("Error on writing data to file");
}
