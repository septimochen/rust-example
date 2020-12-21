#[test]
fn closure_test1() {
    fn function(i: i32) -> i32 {
        i + 1
    }
    let i = 1;
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;
    println!("Testing closures...");
    assert_eq!(function(i), 2);
    assert_eq!(closure_annotated(i), 2);
    assert_eq!(closure_inferred(i), 2);
    println!("Testing passed!");

    let one = || 1;
    assert_eq!(one(), 1);
}
