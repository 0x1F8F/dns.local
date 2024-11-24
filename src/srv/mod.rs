use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

use tracing::info;


pub fn handler(con : TcpStream) {
    
}

pub fn init_tcp( addr : SocketAddrV4 ) {
    info!("starting server: {}",addr);
    let service = TcpListener::bind(addr).unwrap();
    while let Ok((mut con,peer)) = service.accept() {
        info!("connection accepted [ peer address : {} ]", peer);
        handler(con);
    }
}

pub fn init() {
    init_tcp(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8053));
}
