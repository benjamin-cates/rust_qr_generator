mod bits;
mod tests {
    mod test_bits;
    mod test_ec;
    mod test_mask;
    mod test_metadata;
    mod test_qr;
}
pub mod error_correction;
pub mod qr;
mod mask;
mod patterns;
mod message_layout;
mod metadata;

use wasm_bindgen::prelude::*;
use crate::error_correction::ECLevel;

#[wasm_bindgen]
pub fn make_qr(text: &str) -> Box<[i32]> {
    let ec_level = ECLevel::Q;
    let encoding: qr::Encoding = bits::get_encoding(text);
    qr::QR::new(text,encoding,ec_level).bitmap
        .into_iter()
        .map(|vec| vec.into_iter().map(|val| val as i32).collect::<Vec<i32>>())
        .flatten()
        .collect::<Vec<i32>>().into_boxed_slice()


}