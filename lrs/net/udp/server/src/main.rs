use std::error::Error;

const MAX_BUFFER_SIZE: usize = 40960; // 40 KiB

fn main() -> Result<(), Box<dyn Error>> {
    let udp_socket = std::net::UdpSocket::bind("127.0.0.1:50050")?;

    let mut buffer = [0u8; MAX_BUFFER_SIZE];

    loop {
        let (bytes_received, src_addr) = udp_socket.recv_from(&mut buffer)?;
        println!("Received {} bytes from {}", bytes_received, src_addr);
        if bytes_received == 0 {
            println!("No data received, continuing to listen...");
            continue;
        }
        println!("Data: {:?}", &buffer[..bytes_received]);
        // Echo the data back to the sender
        udp_socket.send_to(&buffer[..bytes_received], src_addr)?;
        println!("Echoed {} bytes back to {}", bytes_received, src_addr);
    }
}