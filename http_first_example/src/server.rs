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
        'outer: loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
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
