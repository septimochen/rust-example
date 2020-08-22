#[test]
fn run() {
    let collected_iter: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iter);
    assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], collected_iter)
}