use std::io::{Read, Write};
use std::net::{TcpListener};

fn TcpServer() {
    // Handle the client connection
    let listner = TcpListener::bind("127.0.0.1:9000").unwrap();
    println!("Application LoadBlancer Running on 127.0.0.1:9000");

    for stream in listner.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connection Established");
                // handle client connection seperate thread

                std::thread::spawn(move || {
                    let mut buffer = [0; 1024];

                    stream.read(&mut buffer).unwrap(); // read data from client

                    stream.write_all(b"Hello from server!").unwrap(); // send res to client
                });
            }
            Err(e) => {
                eprintln!("Error Acepting Connection : {}", e)      // error print
            }
        }
    }
}
fn main() {
    TcpServer();
}
