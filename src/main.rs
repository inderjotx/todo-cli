use std::io;

#[derive(PartialEq)]
enum Status {
    Pending,
    Completed,
}
struct Todo {
    status: Status,
    task: String,
}

struct TodoManager {
    todos: Vec<Todo>,
}

impl TodoManager {
    fn add_todo(self: &mut Self, new_todo: Todo) {
        self.todos.push(new_todo);
    }

    fn remove_todo(self: &mut Self, index: usize) {
        self.todos.remove(index);
    }

    fn mark_done(self: &mut Self, index: usize) {
        self.todos[index].status = Status::Completed;
    }

    fn list_all(self: &Self) -> &Vec<Todo> {
        &self.todos
    }

    fn list_done(&self) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|n| n.status == Status::Completed)
            .collect()
    }
    fn list_no_done(&self) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|n| n.status == Status::Pending)
            .collect()
    }
}

fn main() {
    let mut manager = TodoManager { todos: Vec::new() };

    println!("----------- Welcome to the TODO manager -------------- ");
    display_all_command();

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Error reading command");

        match command.trim() {
            "-h" => display_all_command(),
            "-a" => add_todo(&mut manager),
            "-d" => mark_done(&mut manager),
            "-r" => remove_todo(&mut manager),
            "-all" => list_todo(&manager),
            "-ud" => list_done(&manager),
            "-nd" => list_not_done(&manager),
            _ => println!("This is weird"),
        }
    }
}

fn add_todo(manager: &mut TodoManager) {
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

fn remove_todo(manager: &mut TodoManager) {
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

fn mark_done(manager: &mut TodoManager) {
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

fn list_todo(manager: &TodoManager) {
    let todos = manager.list_all();

    for todo in todos {
        match todo.status {
            Status::Completed => println!("✅ - {}", todo.task),
            Status::Pending => println!("❌ - {}", todo.task),
        }
    }
}

fn list_done(manager: &TodoManager) {
    let todos = manager.list_done();

    for todo in todos {
        match todo.status {
            Status::Completed => println!("✅ - {}", todo.task),
            Status::Pending => println!("❌ - {}", todo.task),
        }
    }
}

fn list_not_done(manager: &TodoManager) {
    let todos = manager.list_no_done();

    for todo in todos {
        match todo.status {
            Status::Completed => println!("✅ - {}", todo.task),
            Status::Pending => println!("❌ - {}", todo.task),
        }
    }
}
fn display_all_command() {
    println!("Press -h for help");
    println!("Press -a for adding a todo");
    println!("Press -d for marking a  todo done");
    println!("Press -r for removing a  todo done");
    println!("Press -all to display all todo");
    println!("Press -ud to display done todo");
    println!("Press -ud to display not done todo");
}
