use byteorder::{LittleEndian, WriteBytesExt};
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::TcpStream;

fn main() {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let mut stream = TcpStream::connect(addr).unwrap();

    for x in 0..100 {
        get_server_list(&mut stream, x);
        println!("ssx:{}", x);
    }
}

pub fn get_server_list(socket: &mut TcpStream, x: i32) {
    let mut req = vec![];
    WriteBytesExt::write_i32::<LittleEndian>(&mut req, x).unwrap();
    socket.write(&req).unwrap();
}
