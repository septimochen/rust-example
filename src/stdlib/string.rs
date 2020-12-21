#[test]
fn string_test() {
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("words in reverse: ");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word)
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    let mut string_1 = String::new();
    for c in chars {
        string_1.push(c);
        string_1.push_str(", ");
    }

    println!("{}", string_1);

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str = string_1.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    let mut alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    alice = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
}
