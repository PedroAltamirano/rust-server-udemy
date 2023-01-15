use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    // fn new(addr: String) -> Server {
    pub fn new(addr: String) -> Self {
        // Server { self replace server due to it is the implementation of server struct
        Self {
            // addr: addr     igual que un objeto
            addr,
        }
    }

    pub fn run(self) {
        println!("Listen on {}", self.addr);

        // unwrap, uses the value or finishes program if error
        let listener = TcpListener::bind(&self.addr).unwrap();

        // infinite loops
        // while tre {]
        // loop {}
        // 'label: loop {}
        // continue 'label;
        loop {
            // let res = listener.accept();
            // if res.is_err() {
            //     continue;
            // }
            // let (stream, addr) = res.unwrap();

            // match
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(_request) => {}
                                Err(err) => println!("Failed to parse request: {}", err),
                            }
                            // Request::try_from(&buffer as &[u8]);
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(err) => {
                            println!("Failed to read from connection: {}", err);
                        }
                    }
                    continue;
                }
                Err(err) => {
                    println!("Failed to stablish a connection: {}", err);
                    continue;
                }
            }
        }
    }
}
