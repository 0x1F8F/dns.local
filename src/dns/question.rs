pub fn parse(r : &[u8]) -> Vec<String> {
    let mut name: Vec<String> = Vec::new();
    let mut c:usize = 0;
    loop {
        name.push(
            String::from_utf8(
                r[c+1 ..= c+(r[c] as usize)].to_owned()
            ).unwrap()
        );
        c += r[c] as usize +1;
        if r[c] == 0 {
            break
        }
    }
    name
}



#[cfg(test)]
mod unit_test {
    use crate::dns::question::*;
    #[test]
    fn test_parse_domain() {
        let inp = [ 0x03, 0x77, 0x77, 0x77, 0x07, 0x65, 0x78, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x03, 0x63, 0x6F, 0x6D,0x00 ];
        let out = parse(&inp);
        //print!("{:?}" , out);
        let act_out = vec![
            "www".to_owned(),
            "example".to_owned(),
            "com".to_owned()
        ];
        assert_eq!(out , act_out);
    }
}
