use std::{io::Write, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}};

use tracing::info;


pub fn tcp_handler(con : &mut TcpStream) {
    con.write_all(&[0u8;12]).unwrap();
}

pub fn init_tcp( addr : SocketAddrV4 ) {
    info!("starting tcp listener : {}",addr);
    let service = TcpListener::bind(addr).unwrap();
    while let Ok((mut con,peer)) = service.accept() {
        info!("connection accepted [ peer address : {} ]", peer);
        tcp_handler(&mut con);
    }
}

pub fn init() {
    init_tcp(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8053));
}
