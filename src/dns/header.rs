use crate::bit::get;
use std::fmt::{format, Display};


pub struct Header(pub [u8;12]);

// see: https://datatracker.ietf.org/doc/html/rfc1035
// Header
//  bit:  0         5  6  7  8         11             15
//        +---------------------------------------------+
//        |                       ID                    |
//        +---------------------------------------------|
//        | QR | Opcode | AA | TC | RD | RA | Z | RCODE |
//        +---------------------------------------------|
//        |                     QDCOUNT                 |
//        +---------------------------------------------|
//        |                     ANCOUNT                 |
//        +---------------------------------------------|
//        |                     NSCOUNT                 |
//        +---------------------------------------------|
//        |                     ARCOUNT                 |
//        +---------------------------------------------+


trait ReadHeader {
    fn get_id(&self) -> u16;

    fn get_qr(&self) -> u8;

    fn get_opcode(&self) -> u8;

    fn get_aa(&self) -> u8;

    fn get_tc(&self) -> u8;

    fn get_rd(&self) -> u8;
    
    fn get_ra(&self) -> u8;

    fn get_z(&self) -> u8;

    fn get_rcode(&self) -> u8;
    
    fn get_qdcount(&self) -> u16;

    fn get_ancount(&self) -> u16;
    
    fn get_nscount(&self) -> u16;
    
    fn get_arcount(&self) -> u16;
}

impl ReadHeader for Header {
    fn get_id(&self) -> u16 {
        u16::from_be_bytes(self.0[..2].try_into().unwrap())
    }

    fn get_qr(&self) -> u8 {
        get(self.0[2], 7)
    }

    fn get_opcode(&self) -> u8 {
        let opcode_s = self.0[2];
        (opcode_s >> 3 ) & 0b1111u8
    }

    fn get_aa(&self) -> u8 {
        get(self.0[2] , 2)
    }

    fn get_tc(&self) -> u8 {
        get(self.0[2] , 1)
    }

    fn get_rd(&self) -> u8 {
        get(self.0[2] , 0)
    }
    
    fn get_ra(&self) -> u8 {
        get(self.0[3] , 7)
    }

    fn get_z(&self) -> u8 {
        let z_ = self.0[3];
        (z_ >> 3) & 0b111u8
    }

    fn get_rcode(&self) -> u8 {
        let rcode_ = self.0[3];
        (rcode_ >> 0) & 0b1111u8
    }
    
    fn get_qdcount(&self) -> u16 {
        u16::from_be_bytes(self.0[4..6].try_into().unwrap())
    }

    fn get_ancount(&self) -> u16 {
        u16::from_be_bytes(self.0[6..8].try_into().unwrap())
    }
    
    fn get_nscount(&self) -> u16 {
        u16::from_be_bytes(self.0[8..10].try_into().unwrap())
    }
    
    fn get_arcount(&self) -> u16 {
        u16::from_be_bytes(self.0[10..12].try_into().unwrap())
    }
}

impl Header {
    fn set_id<T>(&mut self, id: &[u8 ;2 ]) {
        self.0[0] = id[0];
        self.0[1] = id[1];

    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.get_id();
        let qr = match self.get_qr() {
            0 => "QUERY",
            1 => "RESPONSE",
            _ => "Error-QA"
        };
        let opcode = match self.get_opcode() {
            0 => "QUERY",
            1 => "IQUERY",
            2 => "STATUS",
            _ => "unreserved"
        };
        let aa = match self.get_aa() {
            0 => "NO-AA",
            1 => "AA",
            _ => "Error-AA"
        };
        let tc = match self.get_tc() {
            0 => "NO-TRUNCATION",
            1 => "TRUNCATION",
            _ => "Error-TC"
        };
        let (rd , ra) = ( self.get_rd()==1 , self.get_ra()==1 );
        let rcode = match self.get_rcode() {
            0 => "No-Error",
            1 => "Format-Error",
            2 => "Server-Error",
            3 => "Name-Error",
            4 => "Not-Implemented-Error",
            5 => "Refused",
            d => &format!("{}-RCODE",d),
        };
        let (ancount , nscount , arcount) = (self.get_ancount() , self.get_nscount() , self.get_arcount());
        write!(f , "id:{} {} {} {} {} rd:{} ra:{} {} ancode:{} nscount:{} arcount:{}" , id,qr,opcode,aa,tc,rd,ra,rcode,ancount,nscount,arcount);
        Ok(())
    }
}
