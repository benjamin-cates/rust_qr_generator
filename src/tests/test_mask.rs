// Tests the mask.rs module

#[test]
fn test_line_penalty() {
    use crate::mask;
    assert_eq!(mask::line_penalty(&vec![vec![1,1,1,1]]),0);
    assert_eq!(mask::line_penalty(&vec![vec![1,1,1,1,1]]),3);
    assert_eq!(mask::line_penalty(&vec![vec![1],vec![1],vec![1],vec![1],vec![1]]),3);
    assert_eq!(mask::line_penalty(&vec![vec![1],vec![1],vec![0],vec![1],vec![1]]),0);
    assert_eq!(mask::line_penalty(&vec![vec![1,1,1,1,1,1]]),4);
    assert_eq!(mask::line_penalty(&vec![vec![1,1,0,1,1,1]]),0);
    assert_eq!(mask::line_penalty(&vec![vec![0,0,0,0,0,1]]),3);
}

#[test]
fn test_square_penalty() {
    use crate::mask;
    assert_eq!(mask::square_penalty(&vec![vec![1,1],vec![1,1]]),3);
    assert_eq!(mask::square_penalty(&vec![vec![1,1,1],vec![1,1,1]]),6);
    assert_eq!(mask::square_penalty(&vec![vec![1,1,1],vec![1,1,1],vec![1,1,1]]),12);
    assert_eq!(mask::square_penalty(&vec![vec![0,0,0],vec![0,0,0],vec![0,0,1]]),9);
    assert_eq!(mask::square_penalty(&vec![vec![0,0],vec![0,0]]),3);
}

#[test]
fn test_finder_penalty() {
    use crate::mask;
    assert_eq!(mask::finder_penalty(&vec![vec![1,0,1,1,1,0,1,0,0,0,0]]),40);
    assert_eq!(mask::finder_penalty(&vec![vec![1,0,1,1,1,0,1,0,0,0,0],vec![1,0,1,1,1,0,1,0,0,0,0]]),80);
    assert_eq!(mask::finder_penalty(&vec![vec![0,0,0,0,1,0,1,1,1,0,1],vec![1,0,1,1,1,0,1,0,0,0,0]]),80);
    assert_eq!(mask::finder_penalty(&vec![vec![1],vec![0],vec![1],vec![1],vec![1],vec![0],vec![1],vec![0],vec![0],vec![0],vec![0]]),40);
    assert_eq!(mask::finder_penalty(&vec![vec![0],vec![0],vec![0],vec![0],vec![1],vec![0],vec![1],vec![1],vec![1],vec![0],vec![1]]),40);
    assert_eq!(mask::finder_penalty(&vec![vec![1,0,1,1,1,0,1,0,0,1,0]]),0);
}

#[test]
fn test_same_color_penalty() {
    use crate::mask;
    assert_eq!(mask::same_color_penalty(&vec![vec![1]]),20);
    assert_eq!(mask::same_color_penalty(&vec![vec![1,1,1,1]]),20);
    assert_eq!(mask::same_color_penalty(&vec![vec![0,0,0,0]]),20);
    assert_eq!(mask::same_color_penalty(&vec![vec![0,0,0,1]]),10);
    assert_eq!(mask::same_color_penalty(&vec![vec![1,1,0,0]]),0);
}

#[test]
fn test_sum_penalty() {
    use crate::mask;
    assert_eq!(mask::sum_penalty(&vec![vec![0,1,0,1,0,1],vec![1,0,1,0,1,0],vec![0,1,0,1,0,1]]),0);
    assert_eq!(mask::sum_penalty(&vec![vec![0,0,0,0,0,0],vec![0,0,0,0,0,0],vec![0,0,0,0,0,0]]),30 + 20 + 12);

}
