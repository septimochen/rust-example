#[test]
pub fn examples_std() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![3, 4, 5];

    assert_eq!(vec1.iter().any(|&x| x == 2), true);
    assert_eq!(vec2.into_iter().any(|x| x == 3), true);
}