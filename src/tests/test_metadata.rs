// Tests the metadata.rs module
#[test]
fn test_blocks_table() {
    use crate::error_correction::ECLevel;
    use crate::metadata::blocks_table_get;
    // Assert num codewords decreaseds with correction level
    for i in 1..=40 {
        assert!(blocks_table_get(i,ECLevel::L).0 > blocks_table_get(i,ECLevel::M).0,"L > M {}",i);
        assert!(blocks_table_get(i,ECLevel::M).0 > blocks_table_get(i,ECLevel::Q).0,"M > Q {}",i);
        assert!(blocks_table_get(i,ECLevel::Q).0 > blocks_table_get(i,ECLevel::H).0,"Q > H {}",i);
    }
    let mut codewords_sum = 0;
    for i in [ECLevel::L,ECLevel::M,ECLevel::Q,ECLevel::H] {
        //Assert num codewords increases with version
        for j in 1..=39 {
            assert!(blocks_table_get(j,i).0 < blocks_table_get(j+1,i).0);
        }
        //Assert num error codes less than 30
        for j in 1..=40 {
            assert!(blocks_table_get(j,i).1 <= 30);
            codewords_sum+=blocks_table_get(j,i).0;
        }
    }
    assert_eq!(codewords_sum,122300);
    
}