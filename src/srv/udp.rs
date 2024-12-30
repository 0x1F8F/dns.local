use std::{io::Error, net::{ToSocketAddrs, UdpSocket}, os::linux::net::SocketAddrExt};

use tracing::trace;

use crate::dns::{header, name};

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
                let name = name::parse_name(&_buf[12..len]);
                trace!("Header => {}", header);
                trace!("Name   => {:?}", name);
            }
            _ => {},
        }
    };
    Ok(())
}
