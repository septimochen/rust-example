#[test]
pub fn path_test() {
    use std::path::Path;
    let path = Path::new(".");
    let _display = path.display();
    println!("{}", _display);
}