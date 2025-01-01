pub mod udp;


use std::{io::Read ,net::TcpStream};
use tracing::{error, info, trace};


pub fn init() {
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


