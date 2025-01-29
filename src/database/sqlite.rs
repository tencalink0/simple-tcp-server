use rusqlite::ffi::sqlite3_index_info_sqlite3_index_constraint_usage;
use rusqlite::{Connection, Result, Error};
use rusqlite::types::FromSql;
use serde_json::Value;
use crate::tools::filesystem::FileSystem;
use crate::database::db::{AQuery, GQuery};
use crate::DatabaseID;
use crate::tools::config::{get_config};

use super::db::DatabaseStruct;

pub struct Sqlite;

impl Sqlite {
    pub fn open(path: &str) -> Result<Connection, Error> {
        match FileSystem::check_file_availability(path.to_string(), "db".to_string()) {
            Some(path_buf) => {
                Connection::open(path_buf)
            },
            None => Err(Error::ExecuteReturnedResults)
        }

    }

    pub fn init(conn: &Connection, this_db: DatabaseStruct) -> bool {
        let table_names: Result<Vec<String>> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .and_then(|mut stmt| {
            stmt.query_map([], |row| row.get(0))
                .and_then(|mapped| mapped.collect())
        });
        let auto_reset_state = get_config("auto_reset")
            .unwrap_or(Value::Bool(false))
            .as_bool()
            .unwrap_or(false);

        let mut status = false;
        if auto_reset_state {
            status = match table_names {
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
        }

        if status && auto_reset_state {
            let sql_string = this_db.items
                .iter()
                .map(|(str, dtype)| {format!("{} {}", str, dtype.as_sql())})
                .collect::<Vec<String>>()
                .join(", ");

            if Self::execute(
                &conn, 
                "CREATE TABLE IF NOT EXISTS users (
                    username TEXT PRIMARY KEY,
                    password TEXT,
                    name TEXT,
                    email TEXT,
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

    pub fn get<T>(conn: &Connection, query: &GQuery) -> Result<Vec<Vec<T>>>
    where
        T: FromSql + Send + 'static,
    {
        let (sql, params) = Self::convertGToSql(query);
        Self::retrieve::<T>(conn, sql.as_str(), Some(params))
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

    pub fn retrieve<T>(conn: &Connection, sql: &str, params: Option<Vec<&dyn rusqlite::ToSql>>) -> Result<Vec<Vec<T>>>
    where
        T: FromSql + Send + 'static,
    {
        let mut stmt = conn.prepare(sql)?;

        let rows = stmt.query_map(params.as_deref().unwrap_or(&[]), |row| {
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

    fn convertAToSql(query: &AQuery) -> (String, Vec<&dyn rusqlite::ToSql>) {
        match query {
            AQuery::User { 
                username, 
                password, 
                name, 
                email, 
                site 
            } => {
                (String::from(
                    "INSERT INTO users (userame, password, name, email, site) VALUES (?1, ?2, ?3, ?4, ?5)"), 
                    vec![username, password, name, email, site]
                )
            },
            AQuery::UserIDAdd { 
                username, 
                password,
                id 
            } => {
                (String::from(
                    "UPDATE users SET id=?1 WHERE username=?2 AND password=?3"), 
                    vec![id, username, password]
                )
            },
            AQuery::UserPing { 
                username, 
                site 
            } => {
                (String::from(
                    "UPDATE users SET site=?1 WHERE username=?2"), 
                    vec![site, username]
                )
            }
        }
    }

    fn convertGToSql(query: &GQuery) -> (String, Vec<&dyn rusqlite::ToSql>) {
        match query {
            GQuery::Password { username } => {
                (String::from("Select password From users WHERE username = ?1"), vec![username as &dyn rusqlite::ToSql])
            },
            GQuery::UserData { username } => {
                (String::from("Select name,email From users WHERE username = ?1"), vec![username as &dyn rusqlite::ToSql])
            }
        }
    }
}