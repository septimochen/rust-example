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

#[test]
fn capture_test() {
    use std::mem;
    let color = String::from("green");
    let print = || println!("'color:' {}", color);
    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };
    inc();
    inc();

    let movable = Box::new(3);
    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };
    consume();
}
