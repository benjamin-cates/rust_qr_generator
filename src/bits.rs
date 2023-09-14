type BitList = Vec<u8>;

/// Pushes bit_count of the lowest bits from value to bit_list
/// Ex:
/// bit_list before: <0,0,1,1,0>
/// push_to_bit_list(bit_list, 0b11001, 5)
/// bit_list after:  <0,0,1,1,0,1,1,0,0,1>
pub(crate) fn push_to_bit_list(bit_list: &mut BitList, value: u32, bit_count: u32) {
    for i in 0..bit_count {
        bit_list.push(
            ((value >> (bit_count - 1 - i)) & 0b00000001)
                .try_into()
                .unwrap(),
        );
    }
}

/// Takes a list of bits and returns the integer value
/// Example: collect_bits([0,1,1,0]) == 6
pub(crate) fn collect_bits(bits: &[u8]) -> usize {
    let mut val: usize = 0;
    for x in bits {
        val = val * 2 + (*x as usize);
    }
    return val;
}

/// Returns the most ideal encoding for the given string
/// If the message is purely numeric, returns Numeric
/// If the message only has alphanumeric chars, returns Alphanumeric
/// If the characters fit in Latin-1, returns Byte
/// Else returns ECI mode for UTF-8
pub(crate) fn get_encoding(str: &str) -> crate::qr::Encoding {
    use crate::qr::Encoding::*;
    if str.chars().all(|ch| ch >= '0' && ch <= '9') {
        return Numeric;
    }
    if str.chars().all(|ch| alphanumeric_char_to_idx(ch).is_some()) {
        return Alphanumeric;
    }
    if str.chars().all(|ch| ch <= 255 as char) {
        return Byte;
    }
    return ECI;
}

/// List of characters in the QR "alphanumeric" mode
#[cfg(test)]
pub const ALPHANUMERIC_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";

/// Returns the index of a character in the QR "alphanumeric" mode
/// Or None if it doesn't exist
pub fn alphanumeric_char_to_idx(ch: char) -> Option<u32> {
    match ch {
        '0'..='9' => Some((ch as u32) - ('0' as u32)),
        'A'..='Z' => Some(10 + (ch as u32) - ('A' as u32)),
        ' ' => Some(36),
        '$' => Some(37),
        '%' => Some(38),
        '*' => Some(39),
        '+' => Some(40),
        '-'..='/' => Some(41 + (ch as u32) - ('-' as u32)),
        ':' => Some(44),
        _ => None,
    }
}

/// Encodes a string into a bit list using QR's "alphanumeric" mode
/// Supported characters are A-Z, 0-9, space, and $%*+-./:
pub(crate) fn encode_alphanumeric(str: &str) -> Result<BitList, char> {
    let mut out: BitList = Vec::with_capacity(str.len() * 11 / 2 + 1);
    for chpair in str.chars().collect::<Vec<char>>().chunks(2) {
        // Pairs of characters are represented with 11 bits
        // where the number is ch0 * 45 + ch1
        if chpair.len() == 2 {
            let number = alphanumeric_char_to_idx(chpair[0]).ok_or(chpair[0])? * 45
                + alphanumeric_char_to_idx(chpair[1]).ok_or(chpair[1])?;
            push_to_bit_list(&mut out, number, 11);
        }
        // Lone chars at the end of the string are represented with 6 bits
        if chpair.len() == 1 {
            push_to_bit_list(&mut out, alphanumeric_char_to_idx(chpair[0]).ok_or(chpair[0])?, 6);
        }
    }
    return Ok(out);
}

/// Decodes valid alphanumeric bit lists into strings
#[cfg(test)]
pub(crate) fn decode_alphanumeric(seq: BitList) -> Option<String> {
    let mut out: String = String::with_capacity(seq.len() / 11 + 1);
    // Chunks into bit sequence of size 11 or a trailing sequence of 6
    for bits in seq.chunks(11) {
        // Get number from bit sequence
        let val = collect_bits(bits);
        //Push decoded characters to string
        if bits.len() == 11 {
            out.push(ALPHANUMERIC_CHARS[val / 45] as char);
            out.push(ALPHANUMERIC_CHARS[val % 45] as char);
        } else if bits.len() == 6 {
            out.push(ALPHANUMERIC_CHARS[val % 45] as char);
        } else {
            return None;
        }
    }
    return Some(String::from(""));
}

/// Converts string of digits to a bit string.
/// Throws error if characters are not all 0-9
pub(crate) fn encode_numeric(str: &str) -> Result<BitList, char> {
    let len = str.len();
    let mut digits: Vec<u32> = Vec::with_capacity(len);
    for ch in str.chars() {
        if ch < '0' || ch > '9' {
            return Err(ch);
        }
        digits.push((ch as u32) - ('0' as u32));
    }
    // Convert digits list to 3 digit list
    let mut out: BitList = Vec::with_capacity(len * 10);
    for dig in digits.chunks(3) {
        if dig.len() == 3 {
            push_to_bit_list(&mut out, dig[0] * 100 + dig[1] * 10 + dig[2], 10);
        } else if dig.len() == 2 {
            push_to_bit_list(&mut out, dig[0] * 10 + dig[1], 7);
        } else if dig.len() == 1 {
            push_to_bit_list(&mut out, dig[0], 4);
        }
    }
    return Ok(out);
}

#[cfg(test)]
const NUMERIC_CHARS: &[u8] = b"0123456789";

#[cfg(test)]
/// Decodes a bitlist into a numeric message
pub(crate) fn decode_numeric(seq: BitList) -> Option<String> {
    let mut out: String = String::from("");
    for bits in seq.chunks(10) {
        let collect = collect_bits(bits);
        // 10 bits corresponds to 3 digits
        if bits.len() == 10 {
            if collect >= 1000 {return None;}
            out.push(NUMERIC_CHARS[collect/100] as char);
            out.push(NUMERIC_CHARS[(collect/10)%10] as char);
            out.push(NUMERIC_CHARS[collect%10] as char);
        }
        // 7 bits corresponds to 2 digits
        else if bits.len() == 7 {
            if collect >= 100 {return None;}
            out.push(NUMERIC_CHARS[(collect/10)%10] as char);
            out.push(NUMERIC_CHARS[collect%10] as char);
        }
        // 4 bits corresponds to 1 digit
        else if bits.len() == 4 {
            if collect >= 10 {return None;}
            out.push(NUMERIC_CHARS[collect%10] as char);
        }
        else {return None;}
    }
    return Some(out);
}

/// Converts UTF-8 string to Latin-1 string encoded as bits
/// Returns invalid character if found (unicode code point > 255)
pub(crate) fn encode_latin(str: &str) -> Result<BitList, char> {
    let mut out: Vec<u8> = Vec::with_capacity(str.len() * 8);
    for ch in str.chars() {
        if ch > 255.into() {
            return Err(ch);
        } else {
            push_to_bit_list(&mut out, ch as u32, 8)
        }
    }
    return Ok(out);
}

#[cfg(test)]
/// Decodes bitlist into latin characters
pub(crate) fn decode_latin(seq: BitList) -> Option<String> {
    let mut out: String = String::with_capacity(seq.len() / 8);
    for bits in seq.chunks(8) {
        let code = collect_bits(bits) as u32;
        out.push(char::from_u32(code).unwrap());
    }
    return Some(out);
}

//pub(crate) fn encode_utf8(str: &str) -> Result<BitList, char> {
//    ECI Coding identity: 0111
//    UTF-8 id: 0d26
//    panic!("Not implemented");
//}

//pub(crate) fn decode_utf8(seq: BitList) -> Option<String> {
//    panic!("Not implemented");
//}

//pub(crate) fn encode_kanji(str: &str) -> Result<BitList, char> {
//    panic!("Not implemented");
//}

//pub(crate) fn decode_kanji(seq: BitList) -> Option<String> {
//    panic!("Not implemented");
//}
