#[derive(PartialEq)]
pub enum Status {
    Pending,
    Completed,
}
pub struct Todo {
    pub status: Status,
    pub task: String,
}

pub struct TodoManager {
    pub todos: Vec<Todo>,
}

impl TodoManager {
    pub fn add_todo(self: &mut Self, new_todo: Todo) {
        self.todos.push(new_todo);
    }

    pub fn remove_todo(self: &mut Self, index: usize) {
        self.todos.remove(index);
    }

    pub fn mark_done(self: &mut Self, index: usize) {
        self.todos[index].status = Status::Completed;
    }

    pub fn list_all(self: &Self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn list_done(&self) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|n| n.status == Status::Completed)
            .collect()
    }
    pub fn list_no_done(&self) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|n| n.status == Status::Pending)
            .collect()
    }
}
