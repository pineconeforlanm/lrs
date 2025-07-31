use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let udp_socket = std::net::UdpSocket::bind("127.0.0.1:50051")?;
    udp_socket.connect("127.0.0.1:50050")?;

    for _ in 0..10 {
        let message = b"Hello, UDP server!";
        udp_socket.send(message)?;
        println!("Sent: {:?}", message);

        let mut buffer = [0u8; 1024];
        let bytes_received = udp_socket.recv(&mut buffer)?;
        println!("Received: {:?}", String::from_utf8_lossy(&buffer[..bytes_received]));
    }

    Ok(())
}