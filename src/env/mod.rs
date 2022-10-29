pub mod env;

// 获取环境变量,可带默认值
#[macro_export]
macro_rules! env_or {
    // 获取env
    ($key:expr) =>{{
        match std::env::var($key) {
           Ok(v) => v.to_string(),
           Err(_) => "".to_string(),
        }
    }};
    // 获取env 带默然值
    ($key:expr, $default:expr) => {{
         match std::env::var($key) {
           Ok(v) => v.to_string(),
           Err(_) => $default.to_string(),
        }
    }};
}

// 自动载入环境变量配置
#[macro_export]
macro_rules! env_autoload {
    () => {
        use dotenv;
        dotenv::dotenv().ok();
    };
    ($path:expr) => {
        use dotenv;
        dotenv::from_filename($path).ok();
    };
}