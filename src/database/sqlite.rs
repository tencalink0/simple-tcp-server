use rusqlite::{Connection, Result, Error};

use crate::database::db::DatabaseType;

use super::{db::Database, textfile::TextFile};
use crate::tools::filesystem::FileSystem;

pub struct Sqlite;

impl Sqlite {
    pub fn open(path: &str) -> Result<Connection, Error> {
        match FileSystem::check_file_availability(path.to_string(), "db".to_string()) {
            Some(path_buf) => {
                match Connection::open(path_buf) {
                    Ok(conn) => {
                        if Self::init(&conn) {
                            Ok(conn)
                        } else {
                            Err(Error::ExecuteReturnedResults)
                        }
                    },
                    Err(e) => Err(e)
                }
            },
            None => Err(Error::ExecuteReturnedResults)
        }

    }

    pub fn add() {

    }

    pub fn init(conn: &Connection) -> bool {
        let table_names: Result<Vec<String>> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .and_then(|mut stmt| {
            stmt.query_map([], |row| row.get(0))
                .and_then(|mapped| mapped.collect())
        });
        let status = match table_names {
            Ok(names) => {
                for table_name in names {
                    if table_name != "sqlite_sequence" {
                        if let Err(_) = conn.execute(&format!("DROP TABLE IF EXISTS {}", table_name), []) {
                            return false; 
                        }
                    }
                }
                true
            }
            Err(_) => false, 
        };
        if status {
            
        }
        status
    }
    
    pub fn do_something(conn: &DatabaseType) -> bool {
        /* 
        match conn {
            DatabaseType::Sqlite(conn) => {
                let mut stmt = conn.prepare("SELECT id, name FROM users");
                match stmt {
                    Ok(ref mut stmt) => {
                        let user_iter = stmt.query_map([], |row| {
                            Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
                        });
                        for user in user_iter {
                            println!("Found user {:?}", user);
                        }
                    },
                    Err(_) => {
                        let user_iter = "";
                    }
                }
                true
            },
            _ => {
                false
            },
        }
        */
        false
    }
}