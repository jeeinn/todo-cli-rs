//! 任务相关定义和操作

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// 任务结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub content: String,
    pub completed: bool,
    #[serde(default)]
    pub completed_at: Option<u64>,
    #[serde(default = "default_current_time")]
    pub created_at: u64,
}

// 为序列化提供默认值
fn default_current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl Task {
    /// 创建新任务
    pub fn new(id: u32, content: String) -> Self {
        Task {
            id,
            content,
            completed: false,
            completed_at: None,
            created_at: default_current_time(),
        }
    }

    /// 标记任务为完成
    pub fn complete(&mut self) {
        self.completed = true;
        let completed_at = default_current_time();
        self.completed_at = Some(completed_at);
    }

    /// 获取创建时间的格式化字符串
    pub fn created_at_formatted(&self) -> String {
        format_timestamp(self.created_at)
    }

    /// 获取完成时间的格式化字符串
    pub fn completed_at_formatted(&self) -> String {
        match self.completed_at {
            Some(timestamp) => format_timestamp(timestamp),
            None => "None".to_string(),
        }
    }
}

/// 格式化时间戳为可读字符串
fn format_timestamp(timestamp: u64) -> String {
    use chrono::{DateTime, Local};

    match chrono::DateTime::from_timestamp(timestamp as i64, 0) {
        Some(dt) => {
            let local_dt: DateTime<Local> = dt.with_timezone(&Local);
            // local_dt.format("%Y-%m-%d %H:%M:%S").to_string()
            local_dt.format("%y-%m-%d %H:%M").to_string()
        }
        None => "None".to_string(),
    }
}