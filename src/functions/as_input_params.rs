#[allow(dead_code)]
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

#[allow(dead_code)]
fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

#[test]
fn params_test() {
    use std::mem;

    let greetings = "hello";
    let mut fareware = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greetings);

        fareware.push_str("!!!");
        println!("Then I screamed {}", fareware);
        println!("Now I can sleep.");
        mem::drop(fareware);
    };

    apply(diary);
}

#[test]
fn params2_test() {
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
