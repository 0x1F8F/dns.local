/// get single bit at given position
pub fn get(x:u8 , pos :u8) -> u8 {
    (x >> pos) & 1
}

/// set bit at position
pub fn set(x: u8, idx: usize, b: bool) -> u8 {
    if b {
        x | (1 << idx)
    } else {
        x & !(1 << idx)
    }
}


#[cfg(test)]
mod test {
    use crate::bit::*;

    #[test]
    fn set_test() {
        let a :u8 = 0b1111_0000u8;
        let b :u8 = set(a , 0, true);
        let b :u8 = set(b , 7, false);
        println!("set : {:08b} == {:08b}", a , b);
        assert_eq!(b , 0b0111_0001u8);
    }

    #[test]
    fn set_test_2() {
        let a = 0b0111_0000u8;
        let b = set(a , 7 , true);
        println!("t2 {:80b} {:80b}", a,b);
        assert_eq!(b , 0b1111_0000u8);
    }

    #[test]
    fn set_test_3() {
        let a = 0b1111_0000u8;
        let b = set(a, 7, false);
        assert_eq!(b , 0b0111_0000u8);
    }

    #[test]
    fn get_test_1() {
        let a :u8 = 0b0111_1111; // 3
        let b = get(a , 0);
        println!("get : {:08b} , {} , {}", b , b , a);
        assert_eq!(b,1u8);
    }
    #[test]
    fn get_test_2() {
        let a = 0b1000_0000u8;
        let b = get(a , 7);
        assert_eq!(b , 1u8);
    }
}
