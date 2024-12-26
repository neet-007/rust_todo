use home::home_dir;
use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    path::PathBuf,
};

pub struct TodoManager {
    path: PathBuf,
}

impl TodoManager {
    fn init(&mut self, new_path: Option<&PathBuf>) -> io::Result<PathBuf> {
        let file_path = home_dir()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")
            })?
            .join(".todo_manager_rust");

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)?;

        match new_path {
            Some(path) => {
                let path_str = path.to_string_lossy();
                f.write_all(path_str.as_bytes())?;
                return Ok(path.to_path_buf());
            }
            None => {
                let mut str = String::new();
                f.read_to_string(&mut str)?;
                return Ok(PathBuf::from(str));
            }
        }
    }

    pub fn new() -> io::Result<TodoManager> {
        let mut todo = TodoManager {
            path: PathBuf::new(),
        };
        todo.path = todo.init(None)?;
        Ok(todo)
    }

    pub fn with_path(path: &PathBuf) -> io::Result<TodoManager> {
        let mut todo = TodoManager {
            path: PathBuf::new(),
        };
        todo.path = todo.init(Some(path))?;
        Ok(todo)
    }

    pub fn set_path(mut self, path: &PathBuf) -> io::Result<Self> {
        self.path = self.init(Some(path))?;
        Ok(self)
    }

    pub fn add_todo(&mut self) {}
    pub fn remove_todo(&mut self) {}
    pub fn mark_done_todo(&mut self) {}
    pub fn mark_important_todo(&mut self) {}
    pub fn list_todos(&mut self) {}
    pub fn change_dir(&mut self) {}
}
