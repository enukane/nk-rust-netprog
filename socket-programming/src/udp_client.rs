use std::net::UdpSocket;
use std::{io, str};

pub fn communicate(address: &str) -> Result<(), failure::Error> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // too long data
        //let mut input = "a".to_string().repeat(70000);

        socket.send_to(input.as_bytes(), address)?;

        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("UDP-Client failed to receive");
        println!("UDP-CLient data='{}'", 
                 str::from_utf8(&buffer).expect("UDP-Client failed to convert to String"));
    }
}