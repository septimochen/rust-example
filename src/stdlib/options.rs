#[allow(dead_code)]
fn checked_dvisision(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

#[test]
pub fn division_test1() {
    let a = checked_dvisision(1.0, 0.00);
    println!("{:?}", a);
    assert_eq!(a, None)
}
