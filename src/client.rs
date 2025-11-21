use crate::config::Server;

use std::net::TcpStream;

fn client_service(server: Server) {

    let server_address : String = server.
    let mut client = TcpStream::connect(server_address);
	
}