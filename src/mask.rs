use crate::qr::QR;
use crate::patterns::PatternMaskType;


impl QR {
    /// Find best mask and apply it
    pub fn apply_masking(&mut self) {
        let bitmap_copy = self.bitmap.clone();
        let mask_fns: Vec<&dyn Fn(usize,usize)->bool> = vec![
            &|x, y| {(x+y)%2 == 0},
            &|_, y| {y%2 == 0},
            &|x, _| {x%3 == 0},
            &|x, y| {(x + y)%3 == 0},
            &|x, y| {(y/2 + x/3)%2 == 0},
            &|x, y| {(x*y)%2 + (x*y)%3 == 0},
            &|x, y| {((x*y)%2+(y*x)%3)%2 == 0},
            &|x, y| {((x+y)%2+(y*x)%3)%2 == 0},
        ];
        let mut best_mask_idx = 0;
        let mut best_mask_penalty = i32::MAX;
        for (i, mask) in mask_fns.iter().enumerate() {
            self.mask_index = i as u8;
            self.format_pattern();
            let pen = self.mask(mask);
            if pen < best_mask_penalty {
                best_mask_idx = i;
                best_mask_penalty = pen;
            }
            self.bitmap = bitmap_copy.clone();
        }
        // Get best masked qr code
        self.mask_index = best_mask_idx as u8;
        self.format_pattern();
        self.mask(mask_fns[best_mask_idx]);
    }

    /// Appy a mask function, xors when func(x,y) returns true and pattern mask is none
    fn mask(&mut self, func: &dyn Fn(usize,usize)->bool) -> i32 {
        let width = self.bitmap.len();
        for x in 0..width {
            for y in 0..width {
                if self.pattern_mask[y][x] == PatternMaskType::None 
                    && func(x,y) {
                        self.bitmap[y][x] ^= 1;
                    }
            }
        }
        return sum_penalty(&self.bitmap);
    }
    
}

/// Returns the sum penalty of a bitmap
pub fn sum_penalty(bitmap: &Vec<Vec<u8>>) -> i32 {
    return line_penalty(bitmap)
        + square_penalty(bitmap)
        + finder_penalty(bitmap)
        + same_color_penalty(bitmap);
}

/// Calculate penalty caused by consecutive lines.
/// Add count-2 to penalty for each consecutive line longer than or equal to 4
/// Consecutive lines can be either in the x or y direction
pub fn line_penalty(bitmap: &Vec<Vec<u8>>) -> i32 {
    let mut counting;
    let mut count = 0;
    let mut penalty = 0;
    for y in 0..bitmap.len() {
        counting = bitmap[y][0];
        for x in 0..bitmap[0].len() {
            if bitmap[y][x] != counting {
                if count >= 5 {penalty+=count-2;}
                counting = bitmap[y][x];
                count=0;
            }
            else {count+=1;}
        }
        if count >= 5 {penalty+=count-2;}
        count = 0;
    }
    count = 0;
    for x in 0..bitmap[0].len() {
        counting = bitmap[0][x];
        for y in 0..bitmap.len() {
            if bitmap[y][x] != counting {
                if count >= 5 {penalty+=count-2;}
                counting = bitmap[y][x];
                count=0;
            }
            else {count+=1;}
        }
        if count >= 5 {penalty+=count-2;}
        count = 0;
    }
    return penalty;
}

/// Returns a penalty of 3 for each 2x2 block with the same color
pub fn square_penalty(bitmap: &Vec<Vec<u8>>) -> i32 {
    let mut penalty = 0;
    for x in 0..(bitmap[0].len()-1) {
        for y in 0..(bitmap.len()-1) {
            if bitmap[y][x] == bitmap[y+1][x] 
                && bitmap[y][x] == bitmap[y][x+1]
                && bitmap[y][x] == bitmap[y+1][x+1] {
                    penalty += 3;
                }
        }
    }
    return penalty;
}

/// Returns occurences of a pattern that looks like the finder pattern
pub fn finder_penalty(bitmap: &Vec<Vec<u8>>) -> i32 {
    let matches = |x: usize, y: usize, dir: usize, buffer: &[u8]| {
        for (i, el) in buffer.iter().enumerate() {
            if bitmap[y+dir*i][x+(1-dir)*i] != *el {return false;}
        }
        return true;
    };
    let mut penalty = 0;
    for (y, vec) in bitmap.iter().enumerate() {
        for x in 0..vec.len() {
            if x+10 < vec.len() && (matches(x,y,0,&[1,0,1,1,1,0,1,0,0,0,0])
                || matches(x,y,0,&[0,0,0,0,1,0,1,1,1,0,1])) {
                    penalty += 40;
                }
            if y+10 < bitmap.len() && (matches(x,y,1,&[1,0,1,1,1,0,1,0,0,0,0])
                || matches(x,y,1,&[0,0,0,0,1,0,1,1,1,0,1])) {
                    penalty += 40;
                }
        }
    }
    return penalty;
}

/// Returns penalty based on the number of colored cells
pub fn same_color_penalty(bitmap: &Vec<Vec<u8>>) -> i32 {
    let mut count: usize = 0;
    for vec in bitmap.iter() {
        for x in vec {
            count += *x as usize;
        }
    }
    let fraction = count as f32 / (bitmap.len()*bitmap[0].len()) as f32 * 100.0;
    return (((fraction/5.0).trunc()-10.0).abs()*2.0) as i32;
}
