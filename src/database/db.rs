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

pub enum DatabaseType {
    Sqlite(Connection),
    Textfile(PathBuf),
    None
}

pub struct Database {
    pub conn: DatabaseType
}

impl Database {
    pub fn connect(fail_safe: bool) -> Self { //Fail safe switches to textfile database if it cannot find the sql server
        match Sqlite::open("logins.db") {
            Ok(connection) => {
                if Sqlite::init(&connection) {
                    Self { conn: DatabaseType::Sqlite(connection) }
                } else {
                    Self { conn: DatabaseType::None }
                }
            },
            Err(_) => {
                if fail_safe {
                    match TextFile::open("logins.txt") {
                        Ok(path_buf) => Self { conn: DatabaseType::Textfile(path_buf) },
                        Err(_) => Self { conn: DatabaseType::None },
                    }
                } else {
                    Self { conn: DatabaseType::None }
                }
            }
        }
    }

    pub fn get_data<T>(&self, sql: &str) -> Result<Vec<Vec<T>>, Box<dyn Error>> 
    where
        T: FromSql + Send + 'static,
    {
        match &self.conn {
            DatabaseType::Sqlite(conn) => {
                match Sqlite::retrieve::<T>(&conn, sql) {
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