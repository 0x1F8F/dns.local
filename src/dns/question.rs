use std::fmt::Display;

use super::{name::Name, types::{Class, Type}};



pub struct Question<'a> {
    name : Name<'a>,
    qclass : QClass,
    qtype : QType,
}

enum QType {
    AXFR = 252,   //          252 A request for a transfer of an entire zone
    MAILB = 253,  //          253 A request for mailbox-related records (MB, MG or MR)
    MAILA = 254,  //          254 A request for mail agent RRs (Obsolete - see MX)
    FB255 = 255,  //          255 A request for all records
}

enum QClass {
    FB255 = 255,  //          255
}

impl Question<'_> {
//    fn new<'b>(data: &'b [u8]) -> Self {
//    let (name, ix ): (Name : u16) = Name::new(data);
//    let q_type
//    let q_class
//    ...
//    }
}

impl Display for QType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",match self {
            Self::AXFR => "AXFR",
            Self::MAILB => "MAILB",
            Self::MAILA => "MAILA",
            Self::FB255 => "T255"
        })
    }
}
