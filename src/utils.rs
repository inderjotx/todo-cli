use super::{Status, Todo, TodoManager, io};

pub fn add_todo(manager: &mut TodoManager) {
    println!("Please type you TODO content");
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("Error reading contnet of new todo");

    manager.add_todo(Todo {
        status: Status::Pending,
        task: message,
    });
}

pub fn remove_todo(manager: &mut TodoManager) {
    println!("Please type index of TODO your want to remove");
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("Error reading contnet of new todo");

    let parsed_index = message
        .trim()
        .parse()
        .expect("error parsing index to number");

    manager.remove_todo(parsed_index);
}

pub fn mark_done(manager: &mut TodoManager) {
    println!("Please type index of TODO your want to mark done");
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("Error reading contnet of new todo");

    let parsed_index = message
        .trim()
        .parse()
        .expect("error parsing index to number");

    manager.mark_done(parsed_index);
}

pub fn list_todo(manager: &TodoManager) {
    let todos = manager.list_all();

    for todo in todos {
        match todo.status {
            Status::Completed => println!("✅ - {}", todo.task),
            Status::Pending => println!("❌ - {}", todo.task),
        }
    }
}

pub fn list_done(manager: &TodoManager) {
    let todos = manager.list_done();

    for todo in todos {
        match todo.status {
            Status::Completed => println!("✅ - {}", todo.task),
            Status::Pending => println!("❌ - {}", todo.task),
        }
    }
}

pub fn list_not_done(manager: &TodoManager) {
    let todos = manager.list_no_done();

    for todo in todos {
        match todo.status {
            Status::Completed => println!("✅ - {}", todo.task),
            Status::Pending => println!("❌ - {}", todo.task),
        }
    }
}
pub fn display_all_command() {
    println!("Press -h for help");
    println!("Press -a for adding a todo");
    println!("Press -d for marking a  todo done");
    println!("Press -r for removing a  todo done");
    println!("Press -all to display all todo");
    println!("Press -dd to display done todo");
    println!("Press -nd to display not done todo");
}
