#[test]
pub fn path_test() {
    use std::path::Path;
    let path = Path::new(".");
    let _display = path.display();
    println!("{}", _display);

    let new_path = path.join("a").join("b");
    match new_path.to_str() {
        None => panic!("new path is not valid utf-8 sequence!"),
        Some(s) => println!("new path is {}", s),
    }
}
