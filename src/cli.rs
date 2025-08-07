//! 命令行参数解析

use clap::{Parser, Subcommand};

/// 命令行应用
#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// 支持的命令
#[derive(Subcommand)]
pub enum Commands {
    /// 添加新任务
    Add { content: String },
    /// 列出所有任务
    List,
    /// 完成任务
    Complete { id: u32 },
    /// 删除任务
    Delete { id: u32 },
}