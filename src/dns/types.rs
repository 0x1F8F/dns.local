//  RR format
//
//                                    1  1  1  1  1  1
//      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                                               |  - var
//    /                                               /
//    /                      NAME                     /
//    |                                               |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                      TYPE                     |  - 2 oct
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                     CLASS                     |  - 2 oct
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                      TTL                      |  - 4 oct
//    |                                               |
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//    |                   RDLENGTH                    |  - 2 oct
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
//    /                     RDATA                     /  - var
//    /                                               /
//    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//
// 
//
//
//

use std::fmt::Display;

pub enum Type {
    A =1,          // 1 host addr
    NS = 2,         // 2 authoritative name server
    MD = 3,         // 3 mail destination
    MF = 4,         // 4 mail fwd
    CNAME = 5,      // 5 alias
    SOA = 6,        // 6 starting zone of authority
    MB = 7,         // 7 mailbox domain name        [alpha]
    MG = 8,         // 8 mail group member          [alpha]
    MR = 9,         // 9 mail rename domain name    [alpha]
    NULL = 10,       // 10 null RR                   [alpha]
    WKS = 11,        // 11 service description
    PTR = 12,        // 12 domain name ptr
    HINFO = 13,      // 13 host inf
    MINFO = 14,      // 14 mailbox / mail list inf
    MX = 15,         // 15 mail
    TXT = 16,        // 16 text
}

pub enum Class {
    IN = 1,     // 1 internet
    CS = 2,     // 2 CSNET
    CH = 3,     // 3 CHAOS
    HS = 4,     // 4 hesiod
}

impl Type {
    fn new( i:u16 ) -> Option<Self> {
        match i {
            1 => Some(Self::A),
            _ => None,
        }
    }
}

impl Class {
    fn new( i:u8 ) -> Option<Self> {
        match i {
            1 => Some(Self::IN),
            _ => None,
        }
    }
}


impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Self::A => "A",
                Self::NS => "NS",
                Self::MD => "MD",
                Self::MF => "MF",
                Self::CNAME => "CNAME",
                Self::SOA => "SOA",
                Self::MB => "MB",
                Self::MG => "MG",
                Self::MR => "MR",
                Self::NULL => "NULL",
                Self::WKS => "WKS",
                Self::PTR => "PTR",
                Self::HINFO => "HINFO",
                Self::MINFO => "MINFO",
                Self::MX => "MX",
                Self::TXT => "TXT",
            }
        )
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Self::IN => "IN",
                Self::CS => "CS",
                Self::CH => "CH",
                Self::HS => "HS",
            }
        )
    }
}
