use std::io::{Read, Write};
use std::net::TcpListener;

const MAX_BUFFER_SIZE: usize = 1024 * 1024; // 1 MB

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Server is listening on port 8080...");

    let mut buffer = [0; MAX_BUFFER_SIZE];

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}