#![allow(dead_code)]
use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;
fn main() {
    /*
     * @author: Adrian Gonzalez Pardo (devnull) gozapaadr@gmail.com
     **/
    /*
     * IP 0.0.0.0 wildcard for listen in every netiface in port 8080
     * You can view your implementation when run with lsof -i:8080 (in Linux rookie)
     **/
    let server = Server::new("0.0.0.0:8080".to_string());
    server.run(WebsiteHandler);
}
