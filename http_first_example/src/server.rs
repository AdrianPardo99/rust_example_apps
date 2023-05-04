use crate::http::Request;
use std::io::{Read, Write};
use std::net::TcpListener;
/*
 * As attrs for consume and use in implementation like OOP
 **/
pub struct Server {
    addr: String,
}

impl Server {
    /*
     * Self is a special keyword for refered to obj `Server`
     * We can set every attr using `attr: variable`
     **/
    pub fn new(addr: String) -> Self {
        return Server { addr: addr };
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    write!(stream, "HTTP/1.1 404 Not Found\r\n\r\n");
                                }
                                Err(e) => println!("Failed to parse request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }

            let result = listener.accept();
            if result.is_err() {
                continue;
            }
            let (stream, addr) = result.unwrap();
        }
    }
}
