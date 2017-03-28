use std::net;
use std::thread;

fn main() {
    let mut socket = net::UdpSocket::bind("127.0.0.1:34234").unwrap();
    let mut buf: [u8; 512] = [97u8, 98u8, 99u8, 100u8];
    socket.send_to(&buf, "66.90.139.210:53");

    if let Ok((_, addr)) = socket.recv_from(&mut buf) {
        println!("Got data: {}", String::from_utf8(buf.to_vec()).unwrap());
        println!("Got data from: {:?}", &addr);
    }
}
