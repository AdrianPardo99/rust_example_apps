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
                    /*
                     * An array can be declared by basic C or C++ way
                     * let a = [1, 2, 3, 4];
                     * let b = ['a', 'b'];
                     * In Rust you can declare an array with a efficient memory allocation
                     * Way 1
                     * let b: [i32;4]; --- It means we allocate integer of 32 bits and we use 4 integers in array
                     * Way 2
                     * let c: [i32;4] = [0,1,2,3];
                     * Way 3
                     * let c = [0;1024]; -- It stores 1024 bytes because 0 sometimes means a u8 value that is equal to 1 byte
                     **/
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer);
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
