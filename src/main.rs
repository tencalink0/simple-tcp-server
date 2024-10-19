use b_glossa::Server;

fn main() {
    println!("WPRLOMG");
    let mut web_server = Server::from_presets();
    web_server.start();
}