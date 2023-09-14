extern crate lazy_static;
// Tests the bits.rs module


#[test]
fn test_numeric() {
    const NUMERIC_TESTS: [&str; 13] = [
        "12345", "54321", "99999", "9999", "999", "99", "9", "1", "0", "00", "000", "0000", "0010000"
    ];
    use crate::bits::*;
    assert_eq!(encode_numeric("5").unwrap(),vec![0,1,0,1]);
    assert_eq!(encode_numeric("10").unwrap(),vec![0,0,0,1,0,1,0]);
    assert_eq!(encode_numeric("512").unwrap(),vec![1,0,0,0,0,0,0,0,0,0]);
    for str in NUMERIC_TESTS {
        assert_eq!(decode_numeric(encode_numeric(&(*str).to_string()).unwrap()).unwrap(),str);
    }
}

#[test]
fn test_alphanumeric() {
    use crate::bits::*;
    // Misc tests
    assert_eq!(encode_alphanumeric("01").unwrap(),vec![0,0,0,0,0,0,0,0,0,0,1]);
    assert_eq!(encode_alphanumeric("21").unwrap(),vec![0,0,0,0,1,0,1,1,0,1,1]);
    assert_eq!(encode_alphanumeric("1").unwrap(),vec![0,0,0,0,0,1]);
    assert_eq!(encode_alphanumeric("001").unwrap(),vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]);
    for x in 0..45 {
        assert_eq!(alphanumeric_char_to_idx(char::from_u32(ALPHANUMERIC_CHARS[x] as u32).unwrap()).unwrap(),x as u32);
    }
    for x in 0..45 {
        for y in 0..45 {
            // Test all two character strings
            let str: String = [
                char::from_u32(ALPHANUMERIC_CHARS[x] as u32).unwrap(),
                char::from_u32(ALPHANUMERIC_CHARS[y] as u32).unwrap()
            ].iter().collect();
            let mut bits: Vec<u8> = vec![];
            push_to_bit_list(&mut bits,(x*45 + y) as u32, 11);
            assert_eq!(encode_alphanumeric(str.as_str()).unwrap(),bits,"{} {}",x,y);
        }
        
        // Test all one character strings
        let str: String = [
            char::from_u32(ALPHANUMERIC_CHARS[x] as u32).unwrap()
        ].iter().collect();
        let mut bits: Vec<u8> = vec![];
        push_to_bit_list(&mut bits,x as u32, 6);
        assert_eq!(encode_alphanumeric(str.as_str()).unwrap(),bits);
        
    }
}

lazy_static::lazy_static! {
    static ref LATIN_CHARS: Vec<u8> = (0..255).collect();
}


#[test]
fn test_latin() {
    const LATIN_TESTS: [&str; 10] = ["ab","4é","\\,,","r~!!","  ","https://leetcode.com","lsajg;kagl;sdgaksl;gjgl;sj","¡","±","ñ"];
    use crate::bits::*;
    assert_eq!(encode_latin("abc").unwrap(),vec![0,1,1,0,0,0,0,1,0,1,1,0,0,0,1,0,0,1,1,0,0,0,1,1]);
    assert_eq!(encode_latin("123").unwrap(),vec![0,0,1,1,0,0,0,1,0,0,1,1,0,0,1,0,0,0,1,1,0,0,1,1]);
    // Test all strings
    for x in 0..255 {
        let ch = char::from_u32(x).unwrap();
        let mut bits: Vec<u8> = vec![];
        push_to_bit_list(&mut bits,x,8);
        assert_eq!(encode_latin(Into::<String>::into(ch).as_str()).unwrap(),bits);
    }
    // Confirm error beyond 255
    for x in 256..1000 {
        let ch = char::from_u32(x).unwrap();
        assert!(encode_latin(Into::<String>::into(ch).as_str()).is_err());
    }
    for str in LATIN_TESTS {
        assert_eq!(decode_latin(encode_latin(&(*str).to_string()).unwrap()).unwrap(),(*str).to_string());
    }
}