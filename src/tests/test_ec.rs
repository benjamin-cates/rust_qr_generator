// Tests the error_correction.rs module

#[test]
fn test_exps() {
    use crate::error_correction as ec;
    assert_eq!(ec::EXPS.0.len(),256);
    assert_eq!(ec::EXPS.1.len(),256);
}

#[test]
fn test_generator_poly() {
    use crate::error_correction as ec;
    for i in 0..31 {
        // Assert polynomial length
        assert_eq!(ec::GENERATOR_POLY[i].len(),i+1);
        // Assert starts with coefficient 1
        assert_eq!(ec::GENERATOR_POLY[i][0],1);
    }
}

#[test]
fn test_ec() {
    use crate::error_correction as ec;
    let str = String::from("https://www.qrcode.com/");
    let bitstream = crate::bits::encode_latin(&str).unwrap();
    let mut message = crate::metadata::get_codewords(&bitstream,str.chars().count(),crate::qr::Encoding::Byte,6,28);
    // Swap trailing byte types because different tutorials conflict on this
    for i in 0..message.len() {
        if message[i] == 17 {message[i] = 236;}
        else if message[i] == 236 {message[i] = 17;}
    }
    // Example provided in https://dev.to/maxart2501/let-s-develop-a-qr-code-generator-part-iii-error-correction-1kbm
    assert_eq!(ec::ec_group(&message,16), vec![52, 61, 242, 187, 29, 7, 216, 249, 103, 87, 95, 69, 188, 134, 57, 20]);
}

#[test]
fn test_ec_grouping() {
    use crate::error_correction as ec;
    let str = String::from("['give you up','let you down','run around and desert you'].map(x=>'Never gonna '+x)");

    let bitstream: Vec<u8> = crate::bits::encode_latin(&str).unwrap();
    let mut message = crate::metadata::get_codewords(&bitstream,str.chars().count(),crate::qr::Encoding::Byte,6, 88);

    // Swap trailing byte types because different tutorials conflict on this
    for i in 0..message.len() {
        if message[i] == 17 {message[i] = 236;}
        else if message[i] == 236 {message[i] = 17;}
    }
    // Example provided in https://dev.to/maxart2501/let-s-develop-a-qr-code-generator-part-ix-structuring-larger-versions-2n5d
    assert_eq!(ec::ec_encode(message,7,ec::ECLevel::Q),
        [69, 2, 118, 6, 117, 34, 53, 114, 226, 22, 210, 6, 178, 194, 114, 230, 230, 118, 118, 118, 194, 66, 214, 246, 118, 198, 119, 6, 23, 230, 151, 87, 39, 70, 2, 230, 102, 66, 86, 87, 135, 18, 82, 7, 226, 54, 131, 2, 7, 150, 6, 87, 211, 114, 150, 247, 23, 39, 226, 183, 247, 82, 38, 66, 116, 130, 82, 6, 247, 7, 230, 144, 7, 70, 86, 150, 87, 236, 87, 247, 230, 247, 102, 17, 66, 82, 87, 236, 63, 55, 231, 201, 50, 250, 102, 104, 200, 194, 61, 125, 26, 180, 168, 254, 126, 223, 192, 39, 134, 237, 34, 82, 65, 63, 187, 55, 69, 173, 106, 47, 177, 234, 241, 7, 117, 63, 145, 100, 48, 84, 90, 98, 96, 80, 78, 65, 107, 121, 18, 27, 111, 79, 88, 60, 5, 26, 172, 186, 138, 158, 22, 131, 26, 176, 42, 140, 155, 124, 136, 125, 103, 124, 40, 135, 187, 15, 127, 157, 35, 125, 76, 150, 227, 245, 86, 196, 251, 62, 86, 16, 253, 37, 71, 64, 189, 243, 248, 199, 7, 15, 1, 181, 202, 64, 199, 23]);
}