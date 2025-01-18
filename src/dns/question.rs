use std::fmt::Display;

use super::{name::Name, types::{Class, Type}};



pub struct Question<'a> {
    qclass : QClass,
    qtype : QType,
    name : Name<'a>,
}

enum QType {
    Type(Type),
    AXFR,   //            252 A request for a transfer of an entire zone
    MAILB,  //           253 A request for mailbox-related records (MB, MG or MR)
    MAILA,  //           254 A request for mail agent RRs (Obsolete - see MX)
    FB255,  //           255 A request for all records
}

enum QClass {
    Class(Class),
    FB255,
}


impl Display for QType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",match self {
            Self::Type(_) => { "" }, // todo: fix
            Self::AXFR => "AXFR",
            Self::MAILB => "MAILB",
            Self::MAILA => "MAILA",
            Self::FB255 => "T255"
        })
    }
}
