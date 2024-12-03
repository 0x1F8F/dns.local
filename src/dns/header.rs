use crate::bit::get;

pub struct Header([u8;12]);


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
