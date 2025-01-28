use std::{error::Error, path::PathBuf};
use rusqlite::types::FromSql;

use crate::database::sqlite::Sqlite;
use rusqlite::Connection;

use super::textfile::TextFile;


pub trait DatabaseConnection {
    fn open() -> bool;
    fn add(query: AQuery) -> bool;
    fn remove() -> bool;
    fn get(query: GQuery) -> Option<String>;
    fn wipe() -> bool;
}

pub enum AQuery {
    User {
        username: String,
        password: String,
        name: Option<String>,
        email: Option<String>,
        site: Option<String>,
    }, // username, password, opt<name>, opt<email>, opt<site>
    UserIDAdd {
        username: String,
        password: String,
        id: String
    },
    UserPing {
        username: String,
        site: String,
    } // username, site
}

pub enum GQuery {
    Password {
        username: String,
    }, // username -> [password]
    UserData {
        username: String,
    } // username -> [name, email]
}

#[derive(Debug)]
pub enum DatabaseType {
    Sqlite(Connection),
    Textfile(PathBuf),
    None
}

#[derive(Debug)]
pub struct Database {
    pub conn: DatabaseType
}

impl Database {
    pub fn connect(fail_safe: bool, file_name: &str) -> Self { //Fail safe switches to textfile database if it cannot find the sql server
        match Sqlite::open(format!("{file_name}.db").as_str()) {
            Ok(connection) => {
                if Sqlite::init(&connection, true) {
                    Self { conn: DatabaseType::Sqlite(connection) }
                } else {
                    Self { conn: DatabaseType::None }
                }
            },
            Err(_) => {
                if fail_safe {
                    match TextFile::open(format!("{file_name}.txt").as_str()) {
                        Ok(path_buf) => Self { conn: DatabaseType::Textfile(path_buf) },
                        Err(_) => Self { conn: DatabaseType::None },
                    }
                } else {
                    Self { conn: DatabaseType::None }
                }
            }
        }
    }

    pub fn get<T>(&self, query: &GQuery) -> Result<Vec<Vec<T>>, Box<dyn Error>> 
    where
        T: FromSql + Send + 'static,
    {
        match &self.conn {
            DatabaseType::Sqlite(conn) => {
                match Sqlite::get::<T>(&conn, query) {
                    Ok(values) => {
                        Ok(values)
                    },
                    Err(e) => {
                        println!("Failed2: {}", e);
                        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed")))
                    }
                }
            },
            _ => {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Database type not implemented")))
            }
        }
    }

    pub fn get_data<T>(&self, sql: &str) -> Result<Vec<Vec<T>>, Box<dyn Error>> 
    where
        T: FromSql + Send + 'static,
    {
        match &self.conn {
            DatabaseType::Sqlite(conn) => {
                match Sqlite::retrieve::<T>(&conn, sql, None) {
                    Ok(values) => {
                        Ok(values)
                    }
                    Err(e) => {
                        println!("Failed {}", e);
                        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
                    }
                }
            },
            _ => {
                println!("Failed");
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed")))
            }
        }   
    }
}