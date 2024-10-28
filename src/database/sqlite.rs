use rusqlite::{Connection, Result, Error};
use rusqlite::types::FromSql;

use crate::tools::filesystem::FileSystem;
use crate::database::db::{AQuery, GQuery};

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
                    username TEXT PRIMARY KEY,
                    password TEXT,
                    name TEXT,
                    email TEXT NOT NULL,
                    site TEXT 
                )",
                []
            ) {
                return Self::execute(
                    &conn,
                    "INSERT INTO users (username, password, name) VALUES (?1, ?2, ?3)",
                    &["admin", "admin123", "Admin"],
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
            let column_count = row.as_ref().column_names().len();
            let mut result_row = Vec::with_capacity(column_count); 

            for i in 0..column_count {
                result_row.push(row.get(i)?); 
            }
            Ok(result_row)
        })?;

        let mut results = Vec::new(); 
        for row in rows {
            let row_data: Vec<T> = row?;
            results.push(row_data); 
        }

        Ok(results) 
    }

    fn convertAToSql(query: &AQuery) -> String {
        match query {
            AQuery::User { 
                username, 
                password, 
                name, 
                email, 
                site 
            } => {
                format!("INSERT INTO ")
            },
            AQuery::UserPing { 
                username, 
                site 
            } => {
                format!("")
            }
        }
    }

    fn convertGToSql(query: &GQuery) -> String {
        match query {
            GQuery::Password { username } => {
                String::from("Select password From users")
            },
            GQuery::UserData { username } => {
                String::from("SELECT password FROM users")
            }
        }
    }
}