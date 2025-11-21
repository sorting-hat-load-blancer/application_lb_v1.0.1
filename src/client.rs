use crate::config::Server;

use std::{io::{Read, Write}, net::TcpStream};

fn client_service(server: Server, message: String) -> std::io::Result<()> {

    let mut client = TcpStream::connect(server.socket_address())?;

    client.write_all(message.as_bytes())?;

    let mut buffer =[0; 1024];
    let read = client.read(&mut buffer)?;
    let res = &buffer[..read];

    println!("Received: {:?}", String::from_utf8_lossy(res));
    Ok(())
    
}