#[allow(dead_code)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn longest_works() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("{}", result);
}

#[test]
fn longest_works_2() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz";

        result = longest(string1.as_str(), string2);
    }
    println!("{}", result);
}
