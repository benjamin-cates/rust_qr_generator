use crate::qr::QR;
use crate::bits;
use crate::error_correction;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum PatternMaskType {
    None = 0,
    Finder = 1,
    Timing = 2,
    Alignment = 3,
    Format = 4,
    Version = 5,
    DarkModule = 6
}

impl QR {
    /// Applies all fixed patterns
    pub fn apply_patterns(&mut self) {
        let length = self.bitmap.len();
        self.create_finder(0,0);
        self.create_finder(0,length-8);
        self.create_finder(length-8,0);
        self.timing_patterns();
        self.alignment_patterns();
        self.format_pattern();
        self.version_information_pattern();
        self.dark_module();
    }
    
    /// Creates a finder pattern with fixed top left position
    fn create_finder(&mut self, left: usize, top: usize) {
        use PatternMaskType::Finder;
        let x_center: usize = if left==0 {3} else {4};
        let y_center: usize = if top==0 {3} else {4};
        for x in 0i32..8 {
            for y in 0i32..8 {
                self.bitmap[y as usize+top][x as usize+left] = (std::cmp::max(
                        (x-x_center as i32).abs(),
                        (y-y_center as i32).abs()
                    ) % 2) .try_into().unwrap();
                self.pattern_mask[y as usize+top][x as usize+left] = Finder;
            }
        }
        self.bitmap[y_center+top][x_center+left] = 1;
    }

    /// Creates alternating timing pattern between the finder patterns
    fn timing_patterns(&mut self) {
        use PatternMaskType::Timing;
        let len = self.bitmap.len();
        for x in 8..(len-8) {
            // Horizontal timing pattern
            self.bitmap[6][x] = (1 - x % 2).try_into().unwrap();
            self.pattern_mask[6][x] = Timing;

            // Vertical timing pattern
            self.bitmap[x][6] = (1 - x % 2).try_into().unwrap();
            self.pattern_mask[x][6] = Timing;
        }
    }

    /// Places all alignment patterns
    fn alignment_patterns(&mut self) {
        if self.version == 1 {return;}
        let n_gaps: i32 = self.version as i32 / 7 + 1;
        let width: i32 = self.bitmap.len() as i32;
        // Width of screen divided by number of gaps rounded up to nearest even number
        let spacing = ((width - 12) as f32 / n_gaps as f32 / 2.0).ceil() as i32 * 2;
        //Alignment patterns on the top and left edges
        for i in 1..n_gaps {
            self.alignment_pattern(width as i32-7-i*spacing,6);
            self.alignment_pattern(6,width as i32-7-i*spacing);
        }
        //Alignment patterns on main grid part
        for i in 0..n_gaps {
            for j in 0..n_gaps {
                self.alignment_pattern(width-7-i*spacing,width-7-j*spacing);
            }
        }
    }

    /// Places singular alignment pattern at (center_x,center_y)
    fn alignment_pattern(&mut self,center_x: i32, center_y: i32) {
        use PatternMaskType::Alignment;
        for x in -2..=2 {
            for y in -2..=2 {
                self.bitmap[(x+center_x) as usize][(y+center_y) as usize] = 
                    (std::cmp::max(x.abs()+1,y.abs()+1) % 2).try_into().unwrap();
                self.pattern_mask[(x+center_x) as usize][(y+center_y) as usize] = Alignment;
            }
        }
        
    }

    /// Places formatting pattern for error correction level and mask id
    pub fn format_pattern(&mut self) {
        use PatternMaskType::Format;
        use error_correction::ECLevel::*;
        let ec = match self.ec_level {
            L => 1,
            M => 0,
            Q => 3,
            H => 2,
        };
        let mut info: Vec<u8> = Vec::with_capacity(15);
        bits::push_to_bit_list(&mut info, ec, 2);
        bits::push_to_bit_list(&mut info, self.mask_index as u32, 3);
        for _ in 0..10 {info.push(0)}
        let format_divisor: Vec<u8> = vec![1,0,1,0,0,1,1,0,1,1,1];
        let format_mask: Vec<u8> = vec![1,0,1,0,1,0,0,0,0,0,1,0,0,1,0];

        // Polynomial division for error correction
        let rest = error_correction::poly_rest(&info, &format_divisor);
        for (i, x) in rest.into_iter().enumerate() {info[i+5] = x;}

        let width = self.bitmap.len();
        for (i, x) in info.iter().enumerate() {
            let bit = *x ^ format_mask[i];
            if i < 7 {
                self.bitmap[8][i + if i==6 {1} else {0}] = bit;
                self.pattern_mask[8][i + if i==6 {1} else {0}] = Format;

                self.bitmap[width-1-i][8] = bit;
                self.pattern_mask[width-1-i][8] = Format;
            }
            else {
                self.bitmap[if i<9 {15} else {14} - i][8] = bit;
                self.pattern_mask[if i<9 {15} else {14} - i][8] = Format;
                
                self.bitmap[8][width-15+i] = bit;
                self.pattern_mask[8][width-15+i] = Format;
            }
        }
    }

    /// Places version information for codes at version 7 or higher
    fn version_information_pattern(&mut self) {
        use PatternMaskType::Version;
        if self.version < 7 {return ();}
        // Version divisor polynomial x^12 + x^11 + ... + x^2 + 1
        let version_divisor: Vec<u8> = vec![1u8,1,1,1,1,0,0,1,0,0,1,0,1];
        // Get version bit_list
        let mut version_bits: Vec<u8> = vec![];
        bits::push_to_bit_list(&mut version_bits,self.version as u32,6);
        // Get error correction
        let ec_version = error_correction::poly_rest(&version_bits,&version_divisor);
        // Place on image
        let width = self.bitmap.len();
        for (i, bit) in version_bits.iter().chain(ec_version.iter()).enumerate() {
            self.bitmap[i/3][i%3 + width-11] = *bit;
            self.pattern_mask[i/3][i%3 + self.bitmap.len()-11] = Version;
            self.bitmap[i%3 + width-11][i/3] = *bit;
            self.pattern_mask[i%3 + self.bitmap.len()-11][i/3] = Version;
        }
    }
    
    /// Places dark module on the top right corner of the bottom left finder pattern
    fn dark_module(&mut self) {
        use PatternMaskType::DarkModule;
        self.bitmap[4 * self.version as usize + 9][8] = 1;
        self.pattern_mask[4 * self.version as usize+ 9][8] = DarkModule;
    }
    
    
}
