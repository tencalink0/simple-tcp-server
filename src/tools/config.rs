use serde_json::Value;
use std::collections::HashMap;
use std::default;
use std::error::Error;
use std::path::Path;
use std::sync::OnceLock;
use crate::tools::filesystem::FileSystem;
use lazy_static::lazy_static;

pub static CONFIG: OnceLock<HashMap<String, Value>> = OnceLock::new();

#[derive(Debug, PartialEq)]
pub enum DType {
    Bool,
    String,
    Integer
}

lazy_static! { // Handling runtime-initialized static data
    pub static ref DATA: Vec<(String, Value, DType)> = vec![
        ("auto_reset".to_string(), Value::Bool(true), DType::Bool),
        ("debug".to_string(), Value::Bool(false), DType::Bool),
    ];
}

fn default_config() -> HashMap<String, Value> {
    println!("Falling back to default");
    let mut presets = HashMap::new();

    for (key, value, dtype) in DATA.iter() {
        presets.insert(key.clone(), value.clone());
    }

    presets
}

pub fn load_config(path: &str) {
    let config = match FileSystem::read_file(Path::new(path)) {
        Ok(data) => match serde_json::from_str::<HashMap<String, Value>>(&data) {
            Ok(json) => json,
            Err(_) => {
                default_config()
            }
        },
        Err(_) => {
            default_config()
        },
    };
    if validate_config(&config) {
        CONFIG.set(config);
    } else {
        CONFIG.set(default_config());
    }
}

pub fn get_config() -> &'static HashMap<String, Value> {
    CONFIG.get().expect("CONFIG is not initialized")
}

fn validate_config(config: &HashMap<String, Value>) -> bool {
    println!("{:?}", config);
    for (key, value, dtype) in DATA.iter() {
        match config.get(key) {
            Some(value) => {
                match value {
                    Value::String(ref s) => {
                        if *dtype != DType::String {
                            return false;
                        }
                    },
                    Value::Bool(b) => {
                        if *dtype != DType::Bool {
                            return false;
                        }
                    },
                    Value::Number(n) => {
                        if let Some(i) = n.as_u64() {
                            if *dtype != DType::Integer {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    },
                    _ => {
                        return false;
                    }
                }

            },
            None => {return false}
        }
    }
    true
}