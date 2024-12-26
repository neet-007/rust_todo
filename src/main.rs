use std::{io, path::PathBuf};

use clap::{CommandFactory, Parser};
use rust_todo::TodoManager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    change_dir: Option<std::path::PathBuf>,

    #[arg(short, long)]
    add: Option<String>,

    #[arg(short, long)]
    remove: Option<String>,

    #[arg(long = "done")]
    mark_done: Option<String>,

    #[arg(long = "important")]
    mark_important: Option<String>,

    #[arg(short, long, default_value_t = false)]
    list: bool,

    #[arg(short = 'p', long, default_value_t = false)]
    repl: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let mut todo_manager = TodoManager::with_path(&PathBuf::from("hello"))?;

    match (
        &args.add,
        &args.remove,
        &args.mark_done,
        &args.mark_important,
    ) {
        (Some(item), None, None, None) => println!("Adding: {}", item),
        (None, Some(item), None, None) => println!("Removing: {}", item),
        (None, None, Some(item), None) => println!("Marking as done: {}", item),
        (None, None, None, Some(item)) => println!("Marking as important: {}", item),
        (None, None, None, None) if args.list => println!("Listing all items..."),
        (None, None, None, None) => {
            Cli::command().print_help().unwrap();
        }
        _ => println!("Please provide only one action at a time"),
    }
    if args.list {
        todo_manager.list_todos();
    }

    if args.repl {
        loop {
            match (
                &args.add,
                &args.remove,
                &args.mark_done,
                &args.mark_important,
            ) {
                (Some(item), None, None, None) => println!("Adding: {}", item),
                (None, Some(item), None, None) => println!("Removing: {}", item),
                (None, None, Some(item), None) => println!("Marking as done: {}", item),
                (None, None, None, Some(item)) => println!("Marking as important: {}", item),
                (None, None, None, None) if args.list => println!("Listing all items..."),
                (None, None, None, None) => {
                    Cli::command().print_help().unwrap();
                    break;
                }
                _ => println!("Please provide only one action at a time"),
            }
            if args.list {
                todo_manager.list_todos();
            }
        }
    }
    Ok(())
}
