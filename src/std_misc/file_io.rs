#[test]
pub fn file_open() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("src/std_misc/hello.txt");
    let display = path.to_str();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {:?}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {:?}: {}", display, why),
        Ok(_) => println!("{:?} contains: \n{}", display, s),
    }
}
