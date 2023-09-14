use crate::bits;
use crate::qr::Encoding;
use crate::qr::QR;
use crate::error_correction::ECLevel;

/// Stores tuple of (number of EC codewords per block, number of blocks) 
/// Accessed by BLOCKS_TABLE[ version ][ ECLevel ]
pub(crate) const BLOCKS_TABLE: [[(usize,usize,usize);4];40] = [
  [(19, 7, 1),   (16, 10, 1),  (13, 13, 1),  (9, 17, 1)],
  [(34, 10, 1),  (28, 16, 1),  (22, 22, 1),  (16, 28, 1)],
  [(55, 15, 1),  (44, 26, 1),  (34, 18, 2),  (26, 22, 2)],
  [(80, 20, 1),  (64, 18, 2),  (48, 26, 2),  (36, 16, 4)],
  [(108, 26, 1),  (86, 24, 2),  (62, 18, 4),  (46, 22, 4)],
  [(136, 18, 2),  (108, 16, 4),  (76, 24, 4),  (60, 28, 4)],
  [(156, 20, 2),  (124, 18, 4),  (88, 18, 6),  (66, 26, 5)],
  [(194, 24, 2),  (154, 22, 4),  (110, 22, 6),  (86, 26, 6)],
  [(232, 30, 2),  (182, 22, 5),  (132, 20, 8),  (100, 24, 8)],
  [(274, 18, 4),  (216, 26, 5),  (154, 24, 8),  (122, 28, 8)],
  [(324, 20, 4),  (254, 30, 5),  (180, 28, 8),  (140, 24, 11)],
  [(370, 24, 4),  (290, 22, 8),  (206, 26, 10), (158, 28, 11)],
  [(428, 26, 4),  (334, 22, 9),  (244, 24, 12), (180, 22, 16)],
  [(461, 30, 4),  (365, 24, 9),  (261, 20, 16), (197, 24, 16)],
  [(523, 22, 6),  (415, 24, 10), (295, 30, 12), (223, 24, 18)],
  [(589, 24, 6),  (453, 28, 10), (325, 24, 17), (253, 30, 16)],
  [(647, 28, 6),  (507, 28, 11), (367, 28, 16), (283, 28, 19)],
  [(721, 30, 6),  (563, 26, 13), (397, 28, 18), (313, 28, 21)],
  [(795, 28, 7),  (627, 26, 14), (445, 26, 21), (341, 26, 25)],
  [(861, 28, 8),  (669, 26, 16), (485, 30, 20), (385, 28, 25)],
  [(932, 28, 8),  (714, 26, 17), (512, 28, 23), (406, 30, 25)],
  [(1006, 28, 9),  (782, 28, 17), (568, 30, 23), (442, 24, 34)],
  [(1094, 30, 9),  (860, 28, 18), (614, 30, 25), (464, 30, 30)],
  [(1174, 30, 10), (914, 28, 20), (664, 30, 27), (514, 30, 32)],
  [(1276, 26, 12), (1000, 28, 21), (718, 30, 29), (538, 30, 35)],
  [(1370, 28, 12), (1062, 28, 23), (754, 28, 34), (596, 30, 37)],
  [(1468, 30, 12), (1128, 28, 25), (808, 30, 34), (628, 30, 40)],
  [(1531, 30, 13), (1193, 28, 26), (871, 30, 35), (661, 30, 42)],
  [(1631, 30, 14), (1267, 28, 28), (911, 30, 38), (701, 30, 45)],
  [(1735, 30, 15), (1373, 28, 29), (985, 30, 40), (745, 30, 48)],
  [(1843, 30, 16), (1455, 28, 31), (1033, 30, 43), (793, 30, 51)],
  [(1955, 30, 17), (1541, 28, 33), (1115, 30, 45), (845, 30, 54)],
  [(2071, 30, 18), (1631, 28, 35), (1171, 30, 48), (901, 30, 57)],
  [(2191, 30, 19), (1725, 28, 37), (1231, 30, 51), (961, 30, 60)],
  [(2306, 30, 19), (1812, 28, 38), (1286, 30, 53), (986, 30, 63)],
  [(2434, 30, 20), (1914, 28, 40), (1354, 30, 56), (1054, 30, 66)],
  [(2566, 30, 21), (1992, 28, 43), (1426, 30, 59), (1096, 30, 70)],
  [(2702, 30, 22), (2102, 28, 45), (1502, 30, 62), (1142, 30, 74)],
  [(2812, 30, 24), (2216, 28, 47), (1582, 30, 65), (1222, 30, 77)],
  [(2956, 30, 25), (2334, 28, 49), (1666, 30, 68), (1276, 30, 8)],
];
/// Get element in the QR code block layout table
/// Returns (number of data codewords, EC codewords per block, number blocks)
pub fn blocks_table_get(version: u8, ec_level: ECLevel) -> (usize,usize,usize) {
    use ECLevel::*;
    return BLOCKS_TABLE[version as usize-1][match ec_level {L => 0, M => 1, Q => 2, H => 3}];
}

/// Return the number of bits needed to encode message length
fn num_length_bits(version: u8, enc: Encoding) -> u32 {
    return match version {
        1..=9 => [10u32,9,8,8],
        10..=26 => [12u32,11,16,10],
        27..=40 => [14u32,13,16,12],
        0 | 41..=u8::MAX => panic!("Version out of bounds")
    }[match enc {
        crate::qr::Encoding::Numeric => 0,
        crate::qr::Encoding::Alphanumeric => 1,
        crate::qr::Encoding::Byte => 2,
        //crate::qr::Encoding::Kanji => 3,
        crate::qr::Encoding::ECI => 2,
    }]
}

/// Given list of encoded bits and metadata, returns bit list with metadata encoded
pub(crate) fn get_codewords(bits: &Vec<u8>, num_chars: usize, enc: Encoding, version: u8, num_codewords: usize) -> Vec<u8> {
    let length_len = num_length_bits(version,enc);
    let mut message_metadata: Vec<u8> = Vec::with_capacity(4+length_len as usize);
    bits::push_to_bit_list(&mut message_metadata,enc as u32,4);
    bits::push_to_bit_list(&mut message_metadata,num_chars as u32,length_len);
    let mut message: Vec<u8> = Vec::with_capacity(num_codewords * 8);
    for x in message_metadata.iter() {message.push(*x);}
    for x in bits.iter() {message.push(*x);}
    let leftover = ((message.len() + 8 - 1)/8)*8 - message.len();
    for _ in 0..leftover {message.push(0);}
    
    // Convert bit list to 8-bit codewords
    let mut message_as_codewords: Vec<u8> = Vec::with_capacity(num_codewords);
    for x in message.chunks(8) {
        message_as_codewords.push(bits::collect_bits(x).try_into().unwrap());
    }
    while message_as_codewords.len() < num_codewords {
        message_as_codewords.push(if message_as_codewords.len() % 2 == 0 {236} else {17});
    }
    return message_as_codewords;
}

impl QR {
    /// Returns the minimum QR version needed to store a message
    pub(crate) fn get_min_version(str: &String, enc: Encoding, ec_level: ECLevel) -> (u8,usize) {
        let num_chars = str.chars().count();
        let num_bits = match enc {
            Encoding::Numeric => num_chars / 3 * 10 
                + (num_chars % 3 == 2) as usize * 7
                + (num_chars % 3 == 1) as usize * 4,
            Encoding::Byte => num_chars * 8,
            Encoding::Alphanumeric => num_chars / 2 * 11
                + (num_chars % 2 == 1) as usize * 6,
            //Encoding::Kanji => num_chars * 13,
            Encoding::ECI => str.len(),
        };

        // Find first version that has enough codewords
        for i in 1..=40 {
            if (num_bits + 4 + num_length_bits(i,enc) as usize) / 8 < blocks_table_get(i,ec_level).0 {
                return (i as u8,blocks_table_get(i,ec_level).0);
            }
        }
        panic!("Cannot fit {} characters with error correction {:?}",num_chars,ec_level);
        
    }
}