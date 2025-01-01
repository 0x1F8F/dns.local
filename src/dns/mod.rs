use header::Header;
use question::Question;

pub mod question;
pub mod header;
pub mod name;
pub mod records;


pub fn parser(payload : &[u8]) -> Option<( Header , Question )> {
    None
}
