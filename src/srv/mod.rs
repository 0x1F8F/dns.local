pub mod udp;


use std::{io::Read ,net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}};

use tracing::{error, info, trace};
use crate::dns::header::Header;
use crate::dns::name::parse_name;

pub fn tcp_handler(con : &mut TcpStream) {
    let data = read_tcp_stream(con);
    let cstr_data = unsafe { 
        std::ffi::CStr::from_bytes_with_nul_unchecked(&data)
    };
    trace!("received : {:?}",&cstr_data);
    let header = parser(&data);
    trace!("q: {}", data[14] );
    let ( name , len ) = parse_name(&data[14..data.len()]);
    trace!("Header => {}",header);
    trace!("Name => {:?} length: {}",name , len);
}

pub fn parser(h : &[u8]) -> Header {
    let f12:Result<[u8; 12], _> = h[0..12].try_into();
    match f12 {
    Ok(h_) => {
        Header(h.as_ref())
    },
    Err(_) => {
            error!("parsing failed due to conv");
            Header(&[0;12]) 
        }
    }
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
    //init_tcp(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8053));
    udp::init("127.0.0.1:8053").unwrap();
}


pub fn read_tcp_stream(stream : &mut TcpStream) -> Vec<u8> {
    let mut byte_stream: Vec<u8> = vec![];
    trace!("Reading packet ...");
    'reader: loop {
        trace!("Entered loop 'reader ");
        let mut buf = [0u8;512];
        match stream.read(&mut buf) {
            Ok(len) => {
                byte_stream.extend_from_slice(&buf[..len]);
                trace!("buffer size {} filled", len);
                if len<511 { break 'reader };
            },
            Err(err) => {
                error!("failed to read stream {}", err);
                break 'reader;
            }
        }
    }
    trace!("Exiting reader ...");
    byte_stream
}


