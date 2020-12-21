#[test]
fn vec_test1() {
    let collected_iter: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iter);
    assert_eq!(vec![0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9], collected_iter);

    let mut xs = vec![1i32, 2, 3];
    xs.push(4);
    assert_eq!(xs, vec![1, 2, 3, 4]);
    assert_eq!(4, xs.len());
    assert_eq!(xs.pop(), Some(4));

    println!("Contents of xs: ");
    for x in xs.iter() {
        println!("< {}", x);
    }
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    assert_eq!(xs, vec![3, 6, 9]);
}
