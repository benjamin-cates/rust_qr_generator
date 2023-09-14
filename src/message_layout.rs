use crate::qr::QR;
use crate::patterns;

impl QR {
    /// Take series of bits and write the message on bitmap
    pub fn write_message(&mut self, message: Vec<u8>) {
        let width = self.bitmap.len();
        let mut pos_x: usize = width-1;
        let mut pos_y: usize = width-1;
        let mut mode = 1;
        // Iterates through each bit
        for bit in message.iter().flat_map(|x| (0..8).map(|el| *x >> ((7-el)) & 1)) {
            loop {
                if mode == 1 {
                    mode = 2;
                    break;
                }
                if mode == 2 {
                    pos_x -= 1;
                    mode = 3;
                }
                else if mode == 3 {
                    // Go left
                    if pos_y == 0 {
                        mode = 5;
                        //Twice if in 8th column
                        if pos_x == 7 { pos_x-=2; }
                        else { pos_x-=1; }
                    }
                    else {
                        pos_y -= 1;
                        pos_x += 1;
                        mode = 2;
                    }
                }
                else if mode == 5 {
                    pos_x -= 1;
                    mode = 6;
                }
                else if mode == 6 {
                    if pos_y == width-1 {
                        // Should not reach this point, means there are not enough modules
                        if pos_x == 0 {
                            panic!("Reached end of QR code");
                        }
                        else {
                            pos_x -= 1;
                            mode = 2;
                        }
                    }
                    else {
                        pos_y += 1;
                        pos_x += 1;
                        mode = 5;
                    }
                }
                if self.pattern_mask[pos_y][pos_x] == patterns::PatternMaskType::None { break; }
            }
            self.bitmap[pos_y][pos_x] = bit;
        }

    }
}