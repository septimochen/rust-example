#[test]
pub fn examples_std() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![3, 4, 5];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    assert_eq!(vec1.iter().any(|&x| x == 2), true);
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    assert_eq!(vec2.into_iter().any(|x| x == 3), true);

    let array1 = [1, 2, 3];
    // let _array2 = [3, 4, 5];
    assert_eq!(array1.iter().any(|&x| x == 2), true);
    // assert_eq!(array2.into_iter().any(|&x| x == 2), false);
}