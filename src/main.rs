use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn TcpServer() {
    // Handle the client connection
    let listner = TcpListener::bind("127.0.0.1:9000").unwrap();
    println!("Application LoadBlancer Running on 127.0.0.1:9000");

    for stream in listner.incoming() {
        match stream {
            Ok(mut stream) => {

            }
            Err(e) => {
                eprintln!("Error Acepting Connection : {}", e)
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
