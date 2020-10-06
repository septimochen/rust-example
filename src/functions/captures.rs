#[test]
fn move_test() {
    let haystack = vec![1, 2, 3];
    let hay = haystack.clone();
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&2));
    assert_eq!(contains(&3), true);

    println!("There're {} elements in vec", hay.len());
}