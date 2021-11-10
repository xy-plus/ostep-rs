use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:10000").unwrap();
    while true {
        let mut message = [0; 1000];
        println!("server:: waiting...");
        let (rc, src_addr) = socket.recv_from(&mut message).unwrap();
        println!(
            "server:: read message [size:{} contents:{}]\n",
            rc,
            std::str::from_utf8(&message).unwrap()
        );
        let rc = socket.send_to(b"goodbye world", src_addr).unwrap();
    }
}
