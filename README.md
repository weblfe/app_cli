## APPCLI 

> 命令行 应用项目 

rust 命令行 - 练手项目

### 基础
> 1. 基础语法 (let,const,&str,String,if-else,fn,type, use,mod,pub,match)
> 2. 常用数据结构 struct,HashMap,
> 3. 常用宏,#[derive(Debug)],#[allow(unused)]

### 初中级
> 1. 项目目录层级
> 2. 模块

> 依赖库
 
```toml
[dependencies]
clap = { version = "3.2.17", features = ["derive"] }
```

> 宏展开 工具


```bash
# 宏展开工具与库安装
cargo +nightly install cargo-expand

# 展开
cargo expand --bin app_cli > app_cli.rs
```