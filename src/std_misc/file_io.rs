use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[test]
pub fn file_open() {
    use std::io::prelude::*;

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

#[test]
pub fn file_create() {
    use std::io::prelude::*;

    const LOREM_IPSUM: &str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

    let path = Path::new("src/std_misc/lorem_ipsum.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {:?}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {:?}: {}", display, why),
        Ok(_) => println!("successfully write to : {}", display),
    }
}

#[allow(dead_code)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
pub fn file_read_lines() {
    if let Ok(lines) = read_lines("src/std_misc/hosts") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}
