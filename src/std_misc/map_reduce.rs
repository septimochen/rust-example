#[test]

pub fn map_test() {
    let data = vec![1, 2, 3];

    // see Struct std::iter::Map
    let b: Vec<i32> = data.clone().into_iter().map(|x| x * 2).collect();
    let s: i32 = data.clone().into_iter().map(|x| x * x).sum();
    println!("{:?}", b);
    println!("{}", s);

    let mut c = 0;

    for pair in vec!['a', 'b', 'c']
        .into_iter()
        .map(|letter| {
            c += 1;
            (letter, c)
        })
        .rev()
    {
        println!("{:?}", pair);
    }
}
