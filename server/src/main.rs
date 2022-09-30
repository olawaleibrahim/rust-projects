use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude;



fn main() {
    let listener = TcpListener::bind(
        "127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        stream.unwrap();
    
        println!("Connection established!");
    }
}
