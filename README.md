## APPCLI 

> 命令行 应用项目 

rust 命令行 - 练手项目


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