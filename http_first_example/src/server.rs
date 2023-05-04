use crate::http::{Request, Response, StatusCode};
use std::io::Read;
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

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1> Its working</h1>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
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
            let (_stream, _addr) = result.unwrap();
        }
    }
}
