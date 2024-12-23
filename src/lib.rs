mod tools;
mod server;
mod login;
mod database;

use std::net::{TcpListener, TcpStream, IpAddr, SocketAddr};
use std::io::prelude::*;
use local_ip_address::local_ip;

use server::response::Response;
use tools::filesystem::FileSystem;
use login::login::Login;
use login::encrypt::{Keys, Encrypt, Decrypt};
use database::db::{Database, AQuery, GQuery};

pub enum State {
    Off, 
    Idle,
    Processing
}

pub struct Server {
    filesystem: FileSystem,
    database: Database,
    pub ip: IpAddr,
    pub port: u16,
    pub state: State,
}

impl Server {
    pub fn new(ip: IpAddr, port_raw: Option<u16>) -> Self {
        let filesystem = FileSystem::init();
        let port = match port_raw {
            Some(num) => num,
            None => 7878
        };        
        let database = Database::connect(false);
        println!("Database {:?}", database);
        Self {
            filesystem,
            database,
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

        let connection_info = Self::get_connection_info(&mut stream);

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
                        } else if this_conn_str == "login" {
                            response.format_file(
                                String::from("index.html")
                            );
                            let query = GQuery::Password { username: (String::from("admin")) };
                            match self.database.get::<String>(&query) {
                                Ok(data) => {
                                    for value in data {
                                        println!("Data {:?}", value)
                                    }
                                },
                                Err(e) => {
                                    println!("Error: {}", e);
                                }
                            }
                            println!("Hello");
                        } else {
                            response.format_file(
                                conn_info.file
                            );
                        }
                    }
                } else {
                    response.format_error();
                }
            },
            None => {
                response.format_error();
            }
        }

        Self::display_connection(&connection_info, &response.status_line);

        stream.write(response.response_data.as_bytes()).unwrap();
        stream.flush().unwrap();
        self.state = State::Idle;
    }

    fn get_connection_info(stream: &mut TcpStream) -> Option<ConnectionData> {
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
            return None;
        }

        let request_type: Vec<String> = request_details[0]
            .split('/')
            .map(|s| s.to_string())
            .collect(); 
        
        if request_type.len() <= 0 {
            return None;
        }

        let request_file: Vec<String> = request_type[1]
            .split(' ')
            .map(|s| s.trim().to_string())
            .collect();
    
        let this_ip = match stream.local_addr() {
            Ok(ip) => Some(ip),
            Err(_) => None
        };

        let connection_info = ConnectionData {
            r#type: request_type[0].trim().to_string(),
            file: request_file[0].trim().to_string(),
            method: request_file[1].trim().to_string(),
            conn_ip: this_ip
        };

        Some(connection_info)
    }

    fn display_connection(connection_info: &Option<ConnectionData>, status_line: &String) {
        let error_404 = String::from("HTTP/1.1 404 NOT FOUND");
        let conn_color = if status_line == &error_404 {
            "\x1b[33m"
        } else { 
            "\x1b[32m"
        };
        match connection_info {
            Some(conn_info) => {
                let addr = match &conn_info.file.as_str() {
                    &"" => String::from("/"),
                    _ => {conn_info.file.clone()}
                };
                match conn_info.conn_ip {
                    Some(ip) => println!("{}{:?} - {}\x1b[0m", conn_color, ip, addr),
                    None => println!("\x1b[31mUnknown - {}\x1b[0m", addr),
                }
            },
            None => println!("\x1b[31mUnknown connection\x1b[0m"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionData {
    pub r#type: String,
    pub file: String,
    pub method: String,
    pub conn_ip: Option<SocketAddr>
}