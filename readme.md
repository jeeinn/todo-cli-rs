# 一个 Rust 实现的 cli todo 

## usage

```bash
PS E:\study\todo_cli> cargo run -- help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\todo_cli.exe help`
A simple todo CLI application

Usage: todo_cli.exe <COMMAND>

Commands:
  add       添加新任务
  list      列出所有任务
  complete  完成任务
  delete    删除任务
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
## demo

```bash
PS E:\study\todo_cli> cargo run -- list
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target\debug\todo_cli.exe list`
No tasks found!


PS E:\study\todo_cli> cargo run -- add "a cli todo, use rust"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\todo_cli.exe add "a cli todo, use rust"`
Task added with ID: 1


PS E:\study\todo_cli> cargo run -- list                      
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\todo_cli.exe list`
Tasks:
ID   State    CreatedAt          CompletedAt        Content
--------------------------------------------------------------------------------
1             25-08-14 19:42                        a cli todo, use rust


PS E:\study\todo_cli> cargo run -- add "just study rust."    
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\todo_cli.exe add "just study rust."`
Task added with ID: 2


PS E:\study\todo_cli> cargo run -- list                  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target\debug\todo_cli.exe list`
Tasks:
ID   State    CreatedAt          CompletedAt        Content
--------------------------------------------------------------------------------
1             25-08-14 19:42                        a cli todo, use rust
2             25-08-14 19:43                        just study rust.


PS E:\study\todo_cli> cargo run -- complete 1                                                                                                                                                                                         
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target\debug\todo_cli.exe complete 1`
Task 1 completed at 25-08-14 19:46!


PS E:\study\todo_cli> cargo run -- list                                                                                                                                                                                               
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target\debug\todo_cli.exe list`
Tasks:
ID   State    CreatedAt          CompletedAt        Content
--------------------------------------------------------------------------------
1    ✓        25-08-14 19:42     25-08-14 19:46     a cli todo, use rust
2             25-08-14 19:43                        just study rust.
```