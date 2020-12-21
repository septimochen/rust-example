#[test]
pub fn threads_test() {
    use std::thread;
    static NTHREADS: i32 = 10;

    let mut children = vec![];
    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }))
    }

    for child in children {
        let _ = child.join();
    }
}
