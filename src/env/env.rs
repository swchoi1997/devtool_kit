use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, OnceLock, RwLock};

use dotenvy::dotenv_iter;
use serde_json::Value;

pub trait EnvSource {
    fn load(&self) -> HashMap<String, String>;
}

pub struct DefaultEnv;
pub struct FileEnv(String);


impl EnvSource for DefaultEnv {
    fn load(&self) -> HashMap<String, String> {
        let mut vars: HashMap<String, String> = HashMap::new();

        for item in dotenv_iter().unwrap() {
            let (key, value) = item.unwrap();
            vars.insert(key, value);
        }
        
        vars
    }
}

impl EnvSource for FileEnv {
    fn load(&self) -> HashMap<String, String> {
        let path = Path::new(&self.0);

        let mut vars: HashMap<String, String> = HashMap::new();

        if path.exists() {
            for item in dotenvy::from_filename_iter(path).unwrap() {
                let (key, value) = item.unwrap();
                vars.insert(key, value);
            }

            vars
        } else {
            HashMap::new()
        }
    }
}

#[derive(Debug)]
pub struct AppEnv {
    env_vars: HashMap<String, String>,
}

impl AppEnv {
    pub fn new<T: EnvSource>(source: T) -> Self {
        AppEnv {
            env_vars: source.load()
        }
    }

    pub fn get_env_vars(&self) -> &HashMap<String, String> {
        &self.env_vars
    }

    pub fn get_var(&self, name: &str) -> &String {
        self.env_vars.get(name)
            .expect(&format!("{} is Not Found", name))
    }

    pub fn get_var_json(&self, name: &str) -> Result<Value, serde_json::Error> {
        let var1: &String = self.get_var(name);

        let processed_str = var1.replace('\'', "\"");


        serde_json::from_str(&processed_str)
    }

    pub fn get_var_json_map<'a>(&'a self, name: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let var1 = self.get_var_json(name)?;

        match var1 {
            Value::Object(map) => {
                let result: HashMap<String, String> = map.into_iter()
                    .filter_map(|(k, v)| {
                        v.as_str().map(|s| (k, s.to_string()))
                    })
                    .collect();
                Ok(result)
            }
            _ => Err("JSON value is not an object".into()),
        }
    }
}

impl Default for AppEnv {
    fn default() -> Self {
        AppEnv::new(DefaultEnv)
    }
}

static GLOBAL_APP_ENV: OnceLock<AppEnv> = OnceLock::new();

static APP_ENVS: OnceLock<RwLock<HashMap<String, Arc<AppEnv>>>> = OnceLock::new();

pub fn load() -> &'static AppEnv {
    GLOBAL_APP_ENV.get_or_init(|| {
        AppEnv::new(DefaultEnv)
    })
}

pub fn load_file(key: &str, filepath: &str) -> Arc<AppEnv> {
    get_app_envs()
        .write()
        .unwrap()
        .entry(key.to_string())
        .or_insert_with(|| {
            Arc::new(AppEnv::new(FileEnv(filepath.to_string())))
        })
        .clone()
}

pub fn load_from_key(key: &str) -> Arc<AppEnv> {
    get_app_envs()
        .read()
        .unwrap()
        .get(key)
        .cloned()
        .unwrap_or_else(|| panic!("No AppEnv found for key: {}", key))
}

fn get_app_envs() -> &'static RwLock<HashMap<String, Arc<AppEnv>>> {
    APP_ENVS.get_or_init(|| RwLock::new(HashMap::new()))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_pagedotenv_test() {
        // let a = AppEnv::new(DefaultEnv);

        let a = AppEnv::new(FileEnv("page.env".to_string()));
        let json_a = a.get_var_json_map("BOOKMARK_PAGE");
        let a = json_a.unwrap();
        println!("{:?}", a);
        println!("{:?}", a.get("icon"));
        println!("{:?}", a.get("title"));
        println!("{:?}", a.get("upper_menu"));
        // println!("{:?}", a.get_var_json_map("HOME_PAGE"));
    }
}
