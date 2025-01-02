
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

pub enum TYPE {
    A,          // 1 host addr
    NS,         // 2 authoritative name server
    MD,         // 3 mail destination
    MF,         // 4 mail fwd
    CNAME,      // 5 alias
    SOA,        // 6 starting zone of authority
    MB,         // 7 mailbox domain name        [alpha]
    MG,         // 8 mail group member          [alpha]
    MR,         // 9 mail rename domain name    [alpha]
    NULL,       // 10 null RR                   [alpha]
    WKS,        // 11 service description
    PTR,        // 12 domain name ptr
    HINFO,      // 13 host inf
    MINFO,      // 14 mailbox / mail list inf
    MX,         // 15 mail
    TXT,        // 16 text
}

pub enum CLASS {
    IN,     // 1 internet
    CS,     // 2 CSNET
    CH,     // 3 CHAOS
    HS,     // 4 hesiod
}

impl TYPE {
    fn new( i:u16 ) -> Option<Self> {
        match i {
            1 => Some(Self::A),
            _ => None,
        }
    }
}

impl CLASS {
    fn new( i:u8 ) -> Option<Self> {
        match i {
            1 => Some(Self::IN),
            _ => None,
        }
    }
}