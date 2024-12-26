use std::path::PathBuf;

pub struct TodoManager {
    path: PathBuf,
}

impl TodoManager {
    pub fn new(path: &PathBuf) -> TodoManager {
        TodoManager { path: path.clone() }
    }
    pub fn add_todo(&mut self) {}
    pub fn remove_todo(&mut self) {}
    pub fn mark_done_todo(&mut self) {}
    pub fn mark_important_todo(&mut self) {}
    pub fn list_todos(&mut self) {}
    pub fn change_dir(&mut self) {}
}
