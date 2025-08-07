//! TodoList CLI 应用主入口

// 声明模块
mod app;
mod cli;
mod error;
mod task;

use clap::Parser;
// 引入模块中的类型
use app::TodoApp;
use cli::{Cli, Commands};
use error::TodoError;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), TodoError> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 创建应用实例
    let mut app = TodoApp::new("myTasks.json")?;

    // 处理命令
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