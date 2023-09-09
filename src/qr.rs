use crate::error_correction;
use crate::bits;
use error_correction::ECLevel;
use crate::metadata;
use crate::patterns::PatternMaskType;

pub struct QR {
    pub bitmap: Vec<Vec<u8>>,
    pub pattern_mask: Vec<Vec<PatternMaskType>>,
    pub ec_level: ECLevel,
    pub version: u8,
    pub mask_index: u8,
}

#[derive(Debug,Copy,Clone)]
pub enum Encoding {
    Numeric = 1,
    Alphanumeric = 2,
    Byte = 4,
    //Kanji = 8,
    ECI = 7,
}

impl QR {
    /// Create a qr code from str using encoding
    pub fn new(str: &String, enc: Encoding, ec_level: ECLevel) -> QR {
        // Get encoded string
        let bits: Vec<u8> = match enc {
            Encoding::Alphanumeric => bits::encode_alphanumeric(&str),
            Encoding::Numeric => bits::encode_numeric(&str),
            Encoding::Byte => bits::encode_latin(&str),
            _ => Err(0 as char),
        }.unwrap_or_else(|ch| panic!("Unrecognized character '{}' in encoding {:?}",ch,enc));
        // Get minimum version
        let (version, num_codewords) = QR::get_min_version(&str,enc, ec_level);
        // Encode message
        let message = metadata::get_codewords(&bits,str.chars().count(),enc,version,num_codewords);
        assert_eq!(message.len(),num_codewords);
        // Apply error correction
        let message_ec = error_correction::ec_encode(message,version,ec_level);

        // Create output code
        let version_size = QR::get_version_size(version);
        let mut out = QR {
            bitmap: vec![vec![0;version_size];version_size],
            pattern_mask: vec![vec![PatternMaskType::None;version_size];version_size],
            version,
            ec_level,
            mask_index: 0,
        };
        out.apply_patterns();
        out.write_message(message_ec);
        out.apply_masking();
        return out;
    }

    /// Get width of a puzzle of size version
    pub fn get_version_size(version: u8) -> usize {
        17 + (version as usize) * 4
    }

    /// Get number of modules that can encode a message
    pub fn get_available_modules(version: usize) -> usize {
        let alignment_count = version / 7 + 2;
        return (version * 4 + 17).pow(2) // Num total blocks
            - 3 * 8 * 8 // Finder patterns
            - if version == 1 {0} else {(alignment_count.pow(2) - 3) * 5 * 5} // Alignment patterns
            - 2 * (version * 4 + 1) // Timing patterns
            + (alignment_count - 2) * 5 * 2 //Add timing patterns overlapped with alignment
            - 2 * 15 // Error and mask info
            - 1 // Dark module
            - if version > 6 {2 * 3 * 6} else {0} // Version format data 
    }
    
    /// Save qr code as an image at file_path
    pub fn to_image(&self, file_path: &str) -> image::ImageResult<()> {
        let size = self.version as usize * 4 + 17 + 8;
        let mut as_u8: Vec<u8> = vec![255;size*size];
        
        for (i, vec) in self.bitmap.iter().enumerate() {
            for (j, el) in vec.iter().enumerate() {
                as_u8[size * (i+4) + (j+4)] = if *el == 1 {0} else {255};
            }
        }
        image::save_buffer(file_path,&as_u8[..],size as u32,size as u32,image::ColorType::L8)

    }
}