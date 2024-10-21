use std::env;
use std::path::{Path, PathBuf};
use std::error::Error;

use std::collections::HashMap;

#[derive(Debug)]
pub struct FileSystem {
    pub allowed_ext: HashMap<String, String>
}

impl FileSystem {
    pub fn init() -> Self {
        let mut allowed_ext: HashMap<String, String> = HashMap::new();
        allowed_ext.insert("html".to_string(),"public".to_string());
        allowed_ext.insert("css".to_string(),"static/css".to_string());
        allowed_ext.insert("js".to_string(),"static/js".to_string());
        Self {
            allowed_ext
        }
    }

    pub fn get_template(&self, string_path: String) -> Option<String> {
        let file_dir = Self::check_file_extension(&self, &string_path);
        let path = match Self::check_file_availability(string_path, file_dir) {
            Some(path) => path,
            None => return None
        };
        match Self::read_file(&path) {
            Ok(file_contents) => Some(file_contents),
            Err(_) => None
        }
    }
    
    fn check_file_extension(&self, string_path: &String) -> String{
        let file_ext: &str = match string_path.split(".").last() {
            Some(file_ext) => file_ext,
            None => ""
        };
        match self.allowed_ext.get(file_ext) {
            Some(path) => return path.clone(),
            None => return String::from("public")
        };
    }

    pub fn check_file_availability(string_path: String, file_dir: String) -> Option<PathBuf> {
        let current_dir = env::current_dir();
        let path: PathBuf = match current_dir {
            Ok(dir) => dir.join(file_dir).join(&string_path),
            Err(_) => return None,
        };
        if path.exists() {
            return Some(path);
        } else {
            return None;
        }
    }

    pub fn read_file(file_name: &Path) -> Result<String, Box<dyn Error>> {
        //println!("FILE PATH: {}", file_path.clone().display());
        std::fs::read_to_string(file_name)
            .map_err(|e| Box::new(e) as Box<dyn Error>) // Maps any errors to Box<dyn Error>
    }
}