use simple_tcp_server::Server;

fn main() {
    let mut web_server = Server::from_presets();
    web_server.start();
}