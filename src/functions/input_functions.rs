fn call_me<f: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I am a function.")
}

#[test]
fn input_run() {
    let closure = || { println!("I am a closure")};

    call_me(function);
    call_me(closure);
}