mod tools;
mod server;
mod login;
mod database;

use std::net::{TcpListener, TcpStream, IpAddr, SocketAddr};
use std::io::prelude::*;
use local_ip_address::local_ip;
use tools::config::load_config;
use std::collections::HashMap;
use serde::{Deserialize};
use serde_json;

use server::response::{Response, ResponseStatus};
use tools::filesystem::FileSystem;
use login::login::Login;
use login::encrypt::{Keys, Encrypt, Decrypt};
use database::db::{Database, AQuery, GQuery};

pub enum State {
    Off, 
    Idle,
    Processing
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum DatabaseID {
    Login,
    Logs,
}

pub struct Server {
    filesystem: FileSystem,
    databases: HashMap<DatabaseID, Database>,
    pub ip: IpAddr,
    pub port: u16,
    pub state: State,
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

impl Server {
    pub fn new(ip: IpAddr, port_raw: Option<u16>) -> Self {
        let filesystem = FileSystem::init();
        let port = match port_raw {
            Some(num) => num,
            None => 7878
        };        

        load_config("config.json");

        let mut databases: HashMap<DatabaseID, Database> = HashMap::new();
        databases.insert(DatabaseID::Login, Database::connect(true, "logins"));
        databases.insert(DatabaseID::Logs, Database::connect(true, "logs"));
        println!("Databases {:?}", databases);

        Self {
            filesystem,
            databases,
            ip,
            port,
            state: State::Off
        }
    }

    pub fn from_presets() -> Self {
        let ip_raw = local_ip();
        let ip = match ip_raw {
            Ok(ip) => ip,
            Err(_) => panic!("Failed to load IP Address!"),
        };
        let port = Some(7878 as u16);
        Self::new(ip, port)
    }

    pub fn start(&mut self) {
        let addr = SocketAddr::new(self.ip, self.port);
        self.state = State::Idle;
        println!("Booting up at: \x1b]8;;http://{:?}\x1b\\{:?}\x1b]8;;\x1b\\", addr, addr);
        let listener = 
            TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            Self::handle_connection(self, stream);
        }
    }

    fn handle_connection(&mut self, mut stream:TcpStream) {
        self.state = State::Processing;

        let (connection_info, raw_connection) = Self::get_connection_info(&mut stream);

        let mut response = Response::new(&self.filesystem);
        match connection_info.clone() {
            Some(conn_info) => {
                if conn_info.r#type == "GET".to_string() {
                    if conn_info.method == "HTTP" {
                        let this_conn_str = conn_info.file.as_str();
                        
                        if this_conn_str == "" {
                            response.format_file(
                                String::from("index.html")
                            );
                        } else {
                            response.format_file(
                                conn_info.file
                            );
                        }
                    }
                } else if conn_info.r#type == "POST".to_string() {
                    let parsed_json: serde_json::Value = serde_json::from_str(&conn_info.body).unwrap();
                    let this_conn_str = conn_info.file.as_str();
                    if this_conn_str == "login" {
                        let username = parsed_json.get("username");
                        let password = parsed_json.get("password");
                        if username.is_none() || password.is_none() {
                            response.format_404();
                        } else {
                            let str_username = username.unwrap().as_str().unwrap();
                            let str_password = password.unwrap().as_str().unwrap();
                            username.unwrap();
                            response.format_file(
                                String::from("index.html")
                            );
                            let query = GQuery::Password { username: str_username.to_string() };
                            let login_database =  self.databases.get(&DatabaseID::Login);
                            match login_database {
                                Some(database) => {
                                    match database.get::<String>(&query) {
                                        Ok(data) => {
                                            let mut login_state = false;
                                            for login in data {
                                                if str_password == login[0] {
                                                    login_state = true;
                                                    break;
                                                }
                                                if login_state {break;}
                                            }
                                            if login_state {
                                                response.format_status("ok");
                                            } else {
                                                response.format_error(403, "Forbidden");
                                            }
        
                                        },
                                        Err(e) => {
                                            println!("Error: {}", e);
                                            response.format_404();
                                        }
                                    }
                                },
                                None => response.format_404()
                            }
                        }
                    } else{
                        response.format_404();
                    }
                } else {
                    response.format_404();
                }
            },
            None => {
                response.format_404();
            }
        }

        Self::display_connection(&connection_info, &response, &raw_connection);
        stream.write(response.response_data.as_bytes()).unwrap();
        stream.flush().unwrap();
        self.state = State::Idle;
    }

    fn get_connection_info(stream: &mut TcpStream) -> (Option<ConnectionData>, Option<Vec<String>>) {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();

        let binding = 
            String::from_utf8_lossy(&buffer[..bytes_read])
                .to_string();
        let request_details: Vec<String> = binding
            .lines()
            .map(|s| s.to_string())
            .collect();
        
        if request_details.len() <= 0 {
            return (None, None);
        }

        let request_type: Vec<String> = request_details[0]
            .split('/')
            .map(|s| s.to_string())
            .collect(); 
        
        if request_type.len() <= 0 {
            return (None, None);
        }

        let request_file: Vec<String> = request_type[1]
            .split(' ')
            .map(|s| s.trim().to_string())
            .collect();
    
        let this_ip = match stream.local_addr() {
            Ok(ip) => Some(ip),
            Err(_) => None
        };

        let body: String = match request_details.last() {
            Some(body) => {body.clone()}
            None => "".to_string()
        };

        let connection_info = ConnectionData {
            r#type: request_type[0].trim().to_string(),
            file: request_file[0].trim().to_string(),
            method: request_file[1].trim().to_string(),
            conn_ip: this_ip,
            body: body
        };

        (Some(connection_info), Some(request_details))
    }

    fn display_connection(connection_info: &Option<ConnectionData>, response: &Response, raw_connection: &Option<Vec<String>>) {
        let conn_color = match response.response_status {
            ResponseStatus::Ok => "\x1b[32m",
            ResponseStatus::Failed => "\x1b[33m",
            ResponseStatus::Denied => "\x1b[31m",
            ResponseStatus::Unknown => "\x1b[34m"
        };
        match connection_info {
            Some(conn_info) => {
                let addr = match &conn_info.file.as_str() {
                    &"" => String::from("/"),
                    _ => {conn_info.file.clone()}
                };
                match conn_info.conn_ip {
                    Some(ip) => println!("{}{:?} - {}\x1b[0m", conn_color, ip, addr),
                    None => println!("{}Unknown - {}\x1b[0m", conn_color, addr),
                }
            },
            None => println!("{}Unknown connection\x1b[0m", conn_color),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionData {
    pub r#type: String,
    pub file: String,
    pub method: String,
    pub conn_ip: Option<SocketAddr>,
    pub body: String
}