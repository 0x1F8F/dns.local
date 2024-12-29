pub fn parse(r : &[u8]) -> ( Vec<String> , u16 ) {
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
    use crate::dns::question::*;
    #[test]
    fn test_parse_domain() {
        let inp: &[u8] = &[
            //+1----2----------4-----------6------------8----------10+
            // [-id--]  [--flag--]  [--QD---]   [--AN---]   [--NS---]
            0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00 ,0x00,
            // [-AR--]---------------[3     W     W     W     7     E
            0x00 ,0x00 ,0x00 ,0x00 ,0x03 ,0x77 ,0x77 ,0x77 ,0x07 ,0x65,
            // X    A     M    P    L      E     3     C     O      M   EOF
            0x78 ,0x61 ,0x6D ,0x70 ,0x6C ,0x65 ,0x03 ,0x63 ,0x6F ,0x6D, 0x00
        ];
        let ( out, a ) = parse(&inp[14..inp.len()]);
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

