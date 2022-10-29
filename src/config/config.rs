use std::fs;
use crate::{env_or};
use serde::{Serialize, Deserialize};

const DEFAULT_DIR: &str = "./conf/config.yaml";
pub const CACHE_DIR_ENV_KEY: &str = "PROTOC_CACHE_DIR";
pub const CONF_FILE_ENV_KEY: &str = "PROTOC_INSTALL_CONF_FILE";

#[allow(unused)]
pub fn build_config(path: Option<&str>) -> Config {
    let file = path.unwrap_or("");
    let mut config = Config::new();
    if file == "" {
        config.load_default();
    } else {
        config.load_by_file(file.to_string());
    }
    return config;
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Config {
    pub version: String,
    pub cache_dir: String,
    pub proxy: Option<Proxy>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Proxy {
    pub host: String,
    pub user: String,
    pub password: String,
}

impl Config {
    /// 构建配置
    /// ```rust
    ///  let mut conf = Config::new();
    ///  conf.load_default();
    /// ```
    pub fn new() -> Config {
        Config {
            proxy: None,
            version: "0.1.0".to_string(),
            cache_dir: env_or!(CACHE_DIR_ENV_KEY),
        }
    }

    #[allow(unused)]
    pub fn load_default(&mut self) -> Result<(), serde_yaml::Error> {
        return self.load_by_file(env_or!(CONF_FILE_ENV_KEY,DEFAULT_DIR));
    }

    #[allow(unused)]
    pub fn load_by_file(&mut self, file: String) -> Result<(), serde_yaml::Error> {
        let content = match fs::read_to_string(file) {
            Err(_) => "".to_string(),
            Ok(data) => data,
        };
        if content.is_empty() {
            return Result::Ok(());
        }
        let c: Config = Config::create_by_yaml(content.as_str());
        self.proxy = c.proxy;
        self.cache_dir = c.cache_dir;
        return Result::Ok(());
    }

    #[allow(unused)]
    pub fn create_by_yaml(json_str: &str) -> Config {
        return serde_yaml::from_str(json_str).unwrap_or(Config::new());
    }

    #[allow(unused)]
    pub fn create_by_json(json_str: &str) -> Config {
        return serde_json::from_str(json_str).unwrap_or(Config::new());
    }

    #[allow(unused)]
    pub fn load_json_file(&mut self, file: String) -> Result<(), serde_json::Error> {
        let content = match fs::read_to_string(file) {
            Err(_) => "".to_string(),
            Ok(data) => data,
        };
        if content.is_empty() {
            return Result::Ok(());
        }
        let c: Config = Config::create_by_json(content.as_str());
        self.proxy = c.proxy;
        self.cache_dir = c.cache_dir;
        return Result::Ok(());
    }

    #[allow(unused)]
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap_or("".to_string())
    }

    #[allow(unused)]
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or("".to_string())
    }

    #[allow(unused)]
    pub fn get_proxy(&self) -> Proxy {
        self.proxy.clone().unwrap_or(Proxy::new())
    }
}

impl Proxy {
    pub fn new() -> Proxy {
        return Proxy {
            host: env_or!("PROTOC_INSTALL_PROXY_HOST"),
            user: env_or!("PROTOC_INSTALL_PROXY_USER"),
            password: env_or!("PROTOC_INSTALL_PROXY_PASSWORD"),
        };
    }
}


#[allow(unused)]
#[cfg(test)]
mod test {
    use crate::env_or;
    use serde_yaml::Error;
    use serde::{Serialize, Deserialize};
    use crate::config::config::{build_config, Config, DEFAULT_DIR, Proxy};

    #[test]
    fn test_parse_config() {
        let mut conf = Config::new();
        match conf.load_default() {
            Ok(v) => {
                println!("version={}", conf.version.as_str());
                println!("cache_dir={}", conf.cache_dir.as_str());
                println!("proxy={:?}", conf.get_proxy());

                assert_eq!(v, (), "解析失败");
                println!();
                println!("yaml={}", conf.to_yaml());
                println!("json={}", conf.to_json());
                println!("config={:?}", Config::create_by_json(conf.to_json().as_str()));
            }
            Err(e) => {
                println!("error={}", e);
                assert!(false, "解析失败");
            }
        };
    }

    #[test]
    fn test_build_config() {
        let conf = build_config(Option::Some(DEFAULT_DIR));
        println!("json={}", conf.to_json());
    }

}

