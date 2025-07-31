use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    stream.write_all("hello server".as_bytes()).unwrap();

    let mut buffer = [0; 1024]; // 1 KB buffer
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("Received {} bytes from server: {:?}", bytes_read, from_utf8(&buffer[0..bytes_read]));

}