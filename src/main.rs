mod bits;
mod tests {
    mod test_bits;
    mod test_ec;
    mod test_mask;
    mod test_metadata;
    mod test_qr;
}
mod error_correction;
mod qr;
mod mask;
mod patterns;
mod message_layout;
mod metadata;
extern crate image;
use std::io::Write;

fn main() -> Result<(),std::io::Error> {

    let mut str = String::new();
    print!("Enter encoded string: ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut str).unwrap();
    // Remove trailing new line
    if str.ends_with('\n') {
        str = str[..str.len()-1].to_string();
    }

    let mut ec_str = String::new();
    print!("Enter error correction level (LMQH): ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut ec_str).unwrap();
    let ec_level = match ec_str.to_lowercase().chars().nth(0).unwrap() {
        'l' => error_correction::ECLevel::L,
        'q' => error_correction::ECLevel::Q,
        'm' => error_correction::ECLevel::M,
        'h' => error_correction::ECLevel::H,
        _ => panic!("Invalid EC format {}",ec_str),
    };

    let encoding: qr::Encoding = bits::get_encoding(str.as_str());

    let file_path = "test.png";
    
    let qr_code = qr::QR::new(&str,encoding,ec_level);
    println!("Encoded string: \"{}\"",str);
    println!("Version: {}",qr_code.version);
    println!("Error correction: {:?}",qr_code.ec_level);
    println!("Number of codewords: {}",metadata::blocks_table_get(qr_code.version,qr_code.ec_level).0);
    println!("Max number of modules: {}", qr::QR::get_available_modules(qr_code.version.into()));
    println!("Mask index: {}",qr_code.mask_index);
    println!("Saved to {}",file_path);

    qr_code.to_image(file_path).expect("");
    return Ok(());
}
