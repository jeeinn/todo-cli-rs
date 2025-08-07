//! 任务相关定义和操作

use serde::{Deserialize, Serialize};

/// 任务结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub content: String,
    pub completed: bool,
}

impl Task {
    /// 创建新任务
    pub fn new(id: u32, content: String) -> Self {
        Task {
            id,
            content,
            completed: false,
        }
    }
}