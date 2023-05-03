use server::Server;

mod http;
mod server;
fn main() {
    /*
     * @author: Adrian Gonzalez Pardo (devnull) gozapaadr@gmail.com
     **/
    /*
     * IP 0.0.0.0 wildcard for listen in every netiface in port 8080
     * You can view your implementation when run with lsof -i:8080 (in Linux rookie)
     **/
    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;
    // let head = Method::HEAD;
    // let connect = Method::CONNECT;
    // let options = Method::OPTIONS;
    // let trace = Method::TRACE;
    // let patch = Method::PATCH;
    let server_host = String::from("0.0.0.0:8080");
    // let server_slice = &server_host[..4];
    // let server_borrow: &str = &server_host; // <- immutable variable
    // let string_literal = "12345"; // <- immutable variable
    // dbg!(&server_host);
    // dbg!(server_slice);
    // dbg!(server_borrow);
    // dbg!(string_literal);
    let server = Server::new(server_host);
    server.run();
}
