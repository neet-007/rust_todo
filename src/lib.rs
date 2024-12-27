use home::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use std::{
    fs::OpenOptions,
    io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    name: String,
    done: bool,
    important: bool,
}

#[derive(Serialize, Deserialize)]
struct TodoFile {
    todos_container: Vec<Todo>,
}

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
        let file_path = home_dir()
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")
            })?
            .join(".todo_manager_rust_todos.json");

        let mut todo = TodoManager {
            path: file_path.clone(),
        };
        todo.path = todo.init(Some(&file_path))?;
        Ok(todo)
    }

    pub fn with_path(path: &PathBuf) -> io::Result<TodoManager> {
        let mut todo = TodoManager {
            path: PathBuf::new(),
        };
        todo.path = todo.init(Some(path))?;
        Ok(todo)
    }

    pub fn set_path(&mut self, path: &PathBuf) -> io::Result<()> {
        self.path = self.init(Some(path))?;
        Ok(())
    }

    pub fn add_todo(&mut self, todo: String) -> io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.path.clone())?;

        let mut todo_file: TodoFile = match from_reader(&file) {
            Ok(parsed) => parsed,
            Err(_) => TodoFile {
                todos_container: Vec::new(),
            },
        };

        todo_file.todos_container.push(Todo {
            name: todo,
            done: false,
            important: false,
        });

        let mut file = file;
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;

        to_writer_pretty(&file, &todo_file)?;

        Ok(())
    }
    pub fn remove_todo(&mut self, todo: String) -> io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.path.clone())?;

        let mut todo_file: TodoFile = match from_reader(&file) {
            Ok(parsed) => parsed,
            Err(_) => TodoFile {
                todos_container: Vec::new(),
            },
        };

        let index = match todo_file
            .todos_container
            .iter()
            .position(|x| *x.name == todo)
        {
            Some(i) => i,
            None => return Ok(()),
        };
        todo_file.todos_container.remove(index);

        let mut file = file;
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;

        to_writer_pretty(&file, &todo_file)?;
        Ok(())
    }
    pub fn mark_done_todo(&mut self, todo: String) -> io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.path.clone())?;

        let mut todo_file: TodoFile = match from_reader(&file) {
            Ok(parsed) => parsed,
            Err(_) => TodoFile {
                todos_container: Vec::new(),
            },
        };

        let index = match todo_file
            .todos_container
            .iter()
            .position(|x| *x.name == todo)
        {
            Some(i) => i,
            None => return Ok(()),
        };

        todo_file
            .todos_container
            .get_mut(index)
            .expect("found the index it must exists")
            .done = true;

        let mut file = file;
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;

        to_writer_pretty(&file, &todo_file)?;

        Ok(())
    }
    pub fn mark_important_todo(&mut self, todo: String) -> io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.path.clone())?;

        let mut todo_file: TodoFile = match from_reader(&file) {
            Ok(parsed) => parsed,
            Err(_) => TodoFile {
                todos_container: Vec::new(),
            },
        };

        let index = match todo_file
            .todos_container
            .iter()
            .position(|x| *x.name == todo)
        {
            Some(i) => i,
            None => return Ok(()),
        };

        todo_file
            .todos_container
            .get_mut(index)
            .expect("found the index it must exists")
            .important = true;

        let mut file = file;
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;

        to_writer_pretty(&file, &todo_file)?;

        Ok(())
    }

    pub fn list_todos(&mut self) -> io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.path.clone())?;

        let todo_file: TodoFile = match from_reader(&file) {
            Ok(parsed) => parsed,
            Err(_) => {
                println!("no todos found");
                return Ok(());
            }
        };

        for todo in &todo_file.todos_container {
            println!(
                "name: {:?} is_done: {:?} is_important: {:?}",
                todo.name, todo.done, todo.important
            );
        }
        Ok(())
    }

    pub fn change_dir(&mut self, new_dir: PathBuf) -> io::Result<()> {
        self.init(Some(&new_dir))?;
        self.path = new_dir.clone();

        Ok(())
    }
}
