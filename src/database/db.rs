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
            Ok(connection) => {println!("SUCCESS1"); Self {conn: DatabaseType::Sqlite(connection)}},
            Err(_) => {
                if fail_safe {
                    match TextFile::open("logins.txt") {
                        Ok(path_buf) => {println!("SUCCESS2"); Self {conn: DatabaseType::Textfile(path_buf)}},
                        Err(_) => {println!("SUCCESS3"); Self {conn: DatabaseType::None}},
                    }
                } else {
                    {println!("SUCCESS4"); Self {conn: DatabaseType::None}}
                }
            }
        }
    }

    pub fn get_data(&self) {
        Sqlite::do_something(&self.conn);
    }
}