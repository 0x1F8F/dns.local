use header::Header;
use question::Question;

pub mod question;
pub mod header;
pub mod name;
pub mod records;
pub mod types;


pub fn query_parser(payload : &[u8]) -> Option<( Header , Question )> {
    None
}
