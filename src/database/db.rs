use std::{error::Error, path::PathBuf};

use crate::database::sqlite::Sqlite;
use rusqlite::Connection;

use super::textfile::TextFile;


pub trait DatabaseConnection {
    fn open() -> bool;
    fn add() -> bool;
    fn remove() -> bool;
    fn get() -> Option<String>;
    fn wipe() -> bool;
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

    pub fn get_data(&self) {
        match &self.conn {
            DatabaseType::Sqlite(conn) => {
                match Sqlite::retrieve::<String>(&conn, "SELECT username FROM users") {
                    Ok(values) => {
                        for value in values {
                            println!("value {:?}",value);
                        }
                    }
                    Err(e) => {
                        println!("Failed {}", e);
                    }
                }
            },
            _ => {
                println!("Failed");
            }
        }   
    }
}