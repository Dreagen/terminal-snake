use std::net::UdpSocket;

pub fn send_data(server_addr: &str, data: &Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.send_to(&data, server_addr)?;

    let mut buf = [0u8; 1024]; 
    let (len, _src) = socket.recv_from(&mut buf)?;

    Ok(buf[..len].to_vec())
}
