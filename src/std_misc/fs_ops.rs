// use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
// use std::os::unix;
use std::path::Path;

#[allow(dead_code)]
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

#[test]
pub fn cat_test() {
    println!("cat file_io.rs");
    match cat(&Path::new("src/std_misc/hello.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }
}
