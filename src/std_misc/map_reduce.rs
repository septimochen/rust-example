#[test]

pub fn map_test() {
    let data = vec![1, 2, 3];

    // see Struct std::iter::Map
    let b: Vec<i32> = data.into_iter().map(|x| x * 2).collect();
    println!("{:?}", b);
}