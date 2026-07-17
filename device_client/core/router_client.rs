use std::net::UdpSocket;

pub fn request_route(context: &String) -> String {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(context.as_bytes(), "127.0.0.1:9000").unwrap();

    let mut buf = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buf).unwrap();
    String::from_utf8_lossy(&buf[..amt]).to_string()
}