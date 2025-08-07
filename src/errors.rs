//! 应用错误类型定义

use std::fmt;

/// 应用错误类型
#[derive(Debug)]
pub enum TodoError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoError::IoError(e) => write!(f, "IO error: {}", e),
            TodoError::JsonError(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl std::error::Error for TodoError {}

// 实现错误类型转换
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