use rusqlite::{Connection, Result, Error};
use rusqlite::types::FromSql;

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
            if Self::execute(
                &conn, 
                "CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL,
                    name TEXT NOT NULL
                )",
                []
            ) {
                return Self::execute(
                    &conn,
                    "INSERT INTO users (name, username, password) VALUES (?1, ?2, ?3)",
                    &["Admin", "admin", "admin123"],
                )
            }
        }
        false
    }
    
    pub fn execute<P: rusqlite::Params>(conn: &Connection, sql: &str, params: P) -> bool {
        let execution_result = conn.execute(
            sql,
            params
        );

        if execution_result.is_err() {
            return false;
        }
        true
    }

    pub fn retrieve<T>(conn: &Connection, sql: &str) -> Result<Vec<Vec<T>>>
    where
    T: FromSql + Send + 'static,
    {
        let mut stmt = conn.prepare(sql)?;

        let rows = stmt.query_map([], |row| {
            let mut result_row = Vec::new();
            let column_count = row.as_ref().column_names().len(); 

            for i in 0..column_count {
                result_row.push(row.get(i)?);
            }
            Ok(result_row)
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }
}