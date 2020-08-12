struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

#[allow(dead_code)]
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

#[allow(dead_code)]
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

#[test]
pub fn run() {
    let x = Val {val: 3.0};
    let y = GenVal {gen_val: "abc"};

    println!("Gen values: {} {}", x.value(), y.value());
    assert_eq!(3.0 as f64, *x.value());
    assert_eq!(&"abc", y.value());
}