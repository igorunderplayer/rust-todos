use std::io::stdin;


fn main() {
    println!("Bem vindo!");
    show_menu_message();

    let mut option = String::new();

    stdin().read_line(&mut option).expect("Burrao");

    if option.trim() == "1" {
        let todos = mytodo::get_all_todos();
        println!();
        println!("Total todos: {}", todos.len());
        for todo in todos {
            println!("[{}] {}", if todo.complete { "X" } else { " " }, todo.title);
        }
    }

    if option.trim() == "2" {
        let ref mut all_todos = mytodo::get_all_todos();
        let mut i = 0;
        for todo in all_todos.iter() {
            i += 1;
            println!("{} - {}", i, todo.title)
        }
        println!("Digite o numero da todo que vc deseja modificar");
        let mut todo_index_string = String::new();
        stdin().read_line(&mut todo_index_string).expect("So coloca um numero");
        let todo_index = todo_index_string.trim().parse::<usize>().expect("o numero precisa ser maior q 0") - 1;
        if todo_index >= all_todos.len() { return };

        let todo = all_todos.get(todo_index as usize).unwrap();
        let mut todo_option = String::new();
        println!("Informações da todo: {}", todo.title);
        println!("Descrição: {}", todo.complete_description);
        println!("Completada: {}", if todo.complete { "Sim" } else { "Não" });
        println!("=============");
        println!("(1) Completar/Descompletar? | (2) Apagar");
        stdin().read_line(&mut todo_option).unwrap();

        if todo_option.trim() == "1" {
            let new_todo = mytodo::Todo {
                title: todo.title.to_string(),
                complete_description: todo.complete_description.to_string(),
                complete: !todo.complete
            };
            all_todos.remove(todo_index);
            all_todos.insert(todo_index, new_todo);
            mytodo::update_todos_file(&all_todos);
        } else if option.trim() == "2" {
            mytodo::delete_todo(todo_index as usize);
        }
    }

    if option.trim() == "3" {
        let mut todo_name = String::new();
        let mut todo_description = String::new();

        println!("Me fala o nome da sua nova todo uhul");
        stdin().read_line(&mut todo_name).expect("Nome burrao");
        
        println!("Me uma desc agr");
        stdin().read_line(&mut todo_description).expect("Desc burrao");

        let new_todo = mytodo::Todo {
            title: todo_name.replace("\n", ""),
            complete_description: todo_description.replace("\n", ""),
            complete: false
        };

        mytodo::create_todo(new_todo);
    }
}

fn show_menu_message() {
    println!("Use uma das opções abaixo:");
    println!("1 | Ver todos os seus todos");
    println!("2 | Ver/editar uma todo");
    println!("3 | Criar uma nova todo");
}
