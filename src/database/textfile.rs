use std::path::PathBuf;
use std::error::Error;

use crate::tools::filesystem::FileSystem;


pub struct TextFile;

impl TextFile {
    pub fn open(path: &str) -> Result<PathBuf, Box<dyn Error>> {
        match FileSystem::check_file_availability(path.to_string(), "db".to_string()) {
            Some(path_buf) => Ok(path_buf),
            None => Err(
                Box::new(
                    std::io::Error::new(
                        std::io::ErrorKind::NotFound, 
                        "File not available"
                    )
                ) as Box<dyn Error>
            ),
        }
    }
}