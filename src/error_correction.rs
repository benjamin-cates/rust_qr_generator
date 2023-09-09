extern crate lazy_static;
use crate::metadata::blocks_table_get;
type Polynomial = Vec<u8>;

#[derive(PartialEq,Eq,Copy,Clone,Debug)]
pub enum ECLevel {
    L,
    M,
    Q,
    H
}

/// Splits data into blocks, add error correction, then combines together
/// Returned result can then be written to the qr code
pub fn ec_encode(message: Vec<u8>, version: u8, ec_level: ECLevel) -> Vec<u8> {
    let (num_data_codewords, ec_per_block, num_blocks) = blocks_table_get(version,ec_level);
    let block_size = num_data_codewords / num_blocks;
    let group1 = num_blocks - num_data_codewords % num_blocks;
    
    let get_block_idx = |idx| {
        idx * block_size + if idx > group1 {idx - group1} else {0}
    };

    let mut out: Vec<u8> = Vec::with_capacity(message.len()+ec_per_block*num_blocks);
    // Add grouped codewords
    for i in 0..block_size {
        for j in 0..num_blocks {
            out.push(message[get_block_idx(j) + i]);
        }
    }
    // Add extraneous group2 codewords
    for i in (group1)..num_blocks {
        out.push(message[get_block_idx(i) + block_size]);
    }
    
    let mut ec_data: Vec<Vec<u8>> = Vec::with_capacity(num_blocks);
    // Get error correction for each group
    for i in 0..num_blocks {
        ec_data.push(ec_group(&message[get_block_idx(i)..get_block_idx(i+1)],ec_per_block));
    }
    //Append error correction codes to output
    for j in 0..ec_per_block {
        for i in 0..num_blocks {
            out.push(ec_data[i][j]);
        }
    }
    return out;
}

/// Returns error correction on group
pub fn ec_group(message: &[u8], ec_count: usize) -> Vec<u8> {
    assert!(ec_count <= 30);
    let mut new_mess = message.to_vec();
    for _ in 0..ec_count {new_mess.push(0);}
    return poly_rest(&new_mess, &GENERATOR_POLY[ec_count]);
}

pub const EXPS: ([u8;256],[u8;256]) = generate_log();
const fn generate_log() -> ([u8;256],[u8;256]) {
    let mut exp: usize = 1;
    let mut value: usize = 1;
    let mut log_out: [u8;256] = [0;256];
    let mut exp_out: [u8;256] = [0;256];
    while exp < 256 {
        value <<= 1;
        if value > 255 {value ^= 285}
        log_out[value] = (exp % 255) as u8;
        exp_out[exp % 255] = value as u8;
        exp += 1;
    }
    return (log_out,exp_out);
}

lazy_static::lazy_static! {
    pub static ref GENERATOR_POLY: Vec<Vec<u8>> = {
        let mut out = Vec::with_capacity(30);
        let mut cur = vec![1];
        out.push(vec![1]);
        for x in 0..30 {
            cur = poly_mul(&cur,&vec![1u8,EXPS.1[x]]);
            out.push(cur.clone());
        }
        out
    };
}

/// Multiplication on GF(256)
pub fn mul(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {0}
    else {EXPS.1[(EXPS.0[a] as usize + EXPS.0[b] as usize) % 255].into()}
}

/// Division on GF(256)
pub fn div(a: usize, b: usize) -> usize {
    return EXPS.1[(EXPS.0[a] as usize + EXPS.0[b] as usize * 254) % 255].into();
}

/// Polynomial multiplication on GF(256)
pub fn poly_mul(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut out: Vec<u8> = vec![0;a.len()+b.len()-1];
    for (i, a_coef) in a.iter().enumerate() {
        for (j, b_coef) in b.iter().enumerate() {
            out[i+j] ^= mul(*a_coef as usize,*b_coef as usize) as u8;
        }
    }
    return out;
}

/// Polynomial remainder on GF(256)
pub fn poly_rest(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut rest = (*a).clone();
    let quotient_len = a.len() - b.len() + 1;
    for x in 0..quotient_len {
        if rest[x] != 0 {
            let factor = div(rest[x] as usize,b[0] as usize);
            let subtr = b.iter().map(|x| mul(*x as usize,factor) as u8).collect::<Vec<u8>>();
            for (i, sub) in subtr.iter().enumerate() {
                rest[x+i] ^= sub;
            }
        }
    }
    return rest.into_iter().skip(quotient_len).collect::<Vec<u8>>();
}

