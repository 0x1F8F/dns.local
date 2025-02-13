use std::{io::Error, net::{ToSocketAddrs, UdpSocket}, os::linux::net::SocketAddrExt};
use tracing::{info, trace};
use crate::dns::{ header::{self , ReadHeader}, name::{self, Name}};

pub fn init<T>( addr: T ) -> Result<(),Error>
where T: ToSocketAddrs
{
    let service = UdpSocket::bind(addr)?;
    let mut buf = [0u8;512];
    while let Ok(( len, peer )) = service.recv_from(&mut buf) {
        match buf.get_mut(..len) {
            Some(_buf) => {
                trace!("conn from : {}", peer);
                let header = header::Header(_buf[..=12].as_ref());
                let name : Name = _buf[12..len].as_ref().into();
                trace!("Header => {}", header);
                process_header(header);
                trace!("{}", name);
            }
            _ => {},
        }
    };
    Ok(())
}

/// 
/// `process_header(header , ... )`
///
pub fn process_header<T>(header: T) where T: ReadHeader {
    if header.get_qr()==0 {
        trace!("getting qustions from peer");
        let q_count: u16 = header.get_qdcount();
        trace!("gathered {} questions and about to procede ",q_count);
    } else {
        info!("Only accepts qustions ... ingnoring peer request");
    }
}
