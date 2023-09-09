#[test]
fn test_get_available_modules() {
    use crate::qr::QR;
    use crate::patterns::PatternMaskType;
    use crate::error_correction::ECLevel;
    for version in 1..=40 {
        let version_size = QR::get_version_size(version.into());
        let mut qr = QR {
            bitmap: vec![vec![0;version_size];version_size],
            pattern_mask: vec![vec![PatternMaskType::None;version_size];version_size],
            version,
            ec_level: ECLevel::L,
            mask_index: 0,
        };
        qr.apply_patterns();
        let mut empty_cells = 0;
        for vec in qr.pattern_mask.iter() {
            for el in vec.iter() {
                if *el == PatternMaskType::None {
                    empty_cells += 1;
                }
            }
        }
        assert_eq!(empty_cells, QR::get_available_modules(version.into()), "Empty cells doesn't match on version {}",version);

    }
}