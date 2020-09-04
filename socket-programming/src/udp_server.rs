use std::net::UdpSocket;
use std::str;

pub fn serve(address: &str) -> Result<(), failure::Error> {
    let server_socket = UdpSocket::bind(address)?;
    loop {
        let mut buf = [0u8; 1024];
        let (size, src) = server_socket.recv_from(&mut buf)?;
        println!("UDP-Server[{}] handling data = '{}'", src, str::from_utf8(&buf[..size])?);
        server_socket.send_to(&buf, src)?;
    }
}