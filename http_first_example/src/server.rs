use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        return Server { addr: addr };
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        'outer: loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream.read();
                }
                Err(e) => println!("Failed to establish a connecton: {}", e),
            }

            let result = listener.accept();
            if result.is_err() {
                continue;
            }
            let (stream, addr) = result.unwrap();
        }
    }
}
