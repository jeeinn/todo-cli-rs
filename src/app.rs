//! 应用核心逻辑

use crate::{error::TodoError, task::Task};
use std::fs;
use std::path::Path;

/// TodoList 应用
#[derive(Debug)]
pub struct TodoApp {
    tasks: Vec<Task>,
    file_path: String,
}

impl TodoApp {
    /// 创建新的 TodoList 应用实例
    pub fn new(file_path: &str) -> Result<Self, TodoError> {
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

    /// 保存任务到文件
    fn save(&self) -> Result<(), TodoError> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    /// 添加任务
    pub fn add_task(&mut self, content: String) -> Result<(), TodoError> {
        let id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let task = Task::new(id, content);
        self.tasks.push(task);
        self.save()?;
        println!("Task added with ID: {}", id);
        Ok(())
    }

    /// 列出所有任务
    pub fn list_tasks(&self) {
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

    /// 完成任务
    pub fn complete_task(&mut self, id: u32) -> Result<(), TodoError> {
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

    /// 删除任务
    pub fn delete_task(&mut self, id: u32) -> Result<(), TodoError> {
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