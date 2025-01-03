
// --- rfc-1034 domain name syntax
//
//  <domain> ::= <subdomain> | " "
//
//  <subdomain> ::= <label> | <subdomain> "." <label>
//
//  <label> ::= <letter> [ [ <ldh-str> ] <let-dig> ]
//
//  <ldh-str> ::= <let-dig-hyp> | <let-dig-hyp> <ldh-str>
//
//  <let-dig-hyp> ::= <let-dig> | "-"
//
//  <let-dig> ::= <letter> | <digit>
//
//  <letter> ::= any one of the 52 alphabetic characters A through Z in
//  upper case and a through z in lower case
//
//  <digit> ::= any one of the ten digits 0 through 9
//
//      Note that while upper and lower case letters are allowed in domain
//  names, no significance is attached to the case.  That is, two names with
//  the same spelling but different case are to be treated as if identical.
//  
//
//

use std::fmt::Display;

use tracing::trace;


pub struct Name<'a> {
    td : &'a [u8],            // Top level domain
    dn : &'a [u8],            // domain name
    sd : Option<&'a [u8]>,    // sub-domain name
}

impl<'b> From<&'b [u8]> for Name<'b> {
    fn from(value: &'b [u8]) -> Name {
        trace!(" loc {} val {}; ",0 , value[0]);
        let a: &'static [u8] = &[1];
        Name { td: a , dn: a , sd : None }
    }
}

impl Display for Name<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f , "Name => ")
    }
}


pub fn parse_name(r : &[u8]) -> ( Vec<String> , u16 ) {
    let mut name: Vec<String> = Vec::new();
    let mut len:u8 = 0;
    let mut buf: String = String::new();
    let mut a = 0;
    for i in r {
        if len!=0 {
            buf.push(char::from_u32(*i as u32).unwrap());
            len-=1;
        } else if *i==0 {
            name.push(buf.clone());
            buf.clear();
            break;
        }
        else {
            if !buf.is_empty() {
                name.push(buf.clone());
                buf.clear();
            }
            len = *i;
            a += *i as u16;
        }
    }
    ( name , a )
}


#[cfg(test)]
mod unit_test {
    use crate::dns::name::*;
    #[test]
    fn test_parse_domain() {
        let inp: &[u8] = &[
            // for tcp payload
            //+1----2----------4-----------6------------8----------10+
            // [-id--]  [--flag--]  [--QD---]   [--AN---]   [--NS---]
            0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
            // [-AR--]---------------[3     W     W     W     7     E
            0x00 ,0x00 ,0x00 ,0x00 ,0x03 ,0x77 ,0x77 ,0x77 ,0x07 ,0x65,
            // X    A     M    P    L      E     3     C     O      M   EOF
            0x78 ,0x61 ,0x6D ,0x70 ,0x6C ,0x65 ,0x03 ,0x63 ,0x6F ,0x6D, 0x00
        ];
        let ( out, a ) = parse_name(&inp[14..inp.len()]);
        //print!("{:?}" , out);
        let act_out = vec![
            "www".to_owned(),
            "example".to_owned(),
            "com".to_owned()
        ];
        assert_eq!(out , act_out);
        assert_eq!(a , 13 );
    }
}

