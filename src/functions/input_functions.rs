#[allow(dead_code)]
fn call_me<F: Fn()>(f: F) {
    f();
}

#[allow(dead_code)]
fn function() {
    println!("I am a function.")
}

#[test]
fn input_run() {
    let closure = || println!("I am a closure");

    call_me(function);
    call_me(closure);
}
