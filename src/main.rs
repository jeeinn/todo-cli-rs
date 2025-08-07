use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    content: String,
    completed: bool,
}

#[derive(Debug)]
struct TodoApp {
    tasks: Vec<Task>,
    file_path: String,
}

// 更好的错误处理方式
#[derive(Debug)]
enum TodoError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

// 实现 Display trait 让错误可以打印
impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TodoError::IoError(e) => write!(f, "IO error: {}", e),
            TodoError::JsonError(e) => write!(f, "JSON error: {}", e),
        }
    }
}

// 实现 Error trait
impl std::error::Error for TodoError {}

impl From<std::io::Error> for TodoError {
    fn from(error: std::io::Error) -> Self {
        TodoError::IoError(error)
    }
}

impl From<serde_json::Error> for TodoError {
    fn from(error: serde_json::Error) -> Self {
        TodoError::JsonError(error)
    }
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { content: String },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
}

impl TodoApp {
    fn new(file_path: &str) -> Result<Self, TodoError> {
        let tasks = if Path::new(file_path).exists() {
            let data = fs::read_to_string(file_path)?;
            serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        Ok(TodoApp {
            tasks,
            file_path: file_path.to_string(),
        })
    }

    fn save(&self) -> Result<(), TodoError> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    fn add_task(&mut self, content: String) -> Result<(), TodoError> {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let task = Task {
            id,
            content,
            completed: false,
        };
        self.tasks.push(task);
        self.save()?;
        println!("Task added with ID: {}", id);
        Ok(())
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found!");
            return;
        }

        println!("Tasks:");
        for task in &self.tasks {
            let status = if task.completed { "✓" } else { " " };
            println!("[{}] {} {}", status, task.id, task.content);
        }
    }

    fn complete_task(&mut self, id: u32) -> Result<(), TodoError> {
        let task = self.tasks.iter_mut().find(|t| t.id == id);
        match task {
            Some(t) => {
                t.completed = true;
                self.save()?;
                println!("Task {} completed!", id);
                Ok(())
            }
            None => {
                println!("Task {} not found!", id);
                Ok(())
            }
        }
    }

    fn delete_task(&mut self, id: u32) -> Result<(), TodoError> {
        let len_before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);

        if self.tasks.len() < len_before {
            self.save()?;
            println!("Task {} deleted!", id);
            Ok(())
        } else {
            println!("Task {} not found!", id);
            Ok(())
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), TodoError> {
    let cli = Cli::parse();
    let mut app = TodoApp::new("tasks.json")?;

    match &cli.command {
        Commands::Add { content } => {
            app.add_task(content.clone())?;
        }
        Commands::List => {
            app.list_tasks();
        }
        Commands::Complete { id } => {
            app.complete_task(*id)?;
        }
        Commands::Delete { id } => {
            app.delete_task(*id)?;
        }
    }

    Ok(())
}