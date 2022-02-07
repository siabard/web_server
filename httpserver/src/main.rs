mod handler;
mod router;
mod server;
use server::Server;

fn main() {
    // start server
    let server = Server::new("localhost:3000");
    // run server
    server.run();
}
