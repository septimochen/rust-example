struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

#[allow(dead_code)]
fn fibo() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

#[test]
pub fn iter_1() {
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    assert_eq!(sequence.next(), None)
}

#[test]
pub fn iter_2() {
    let mut sequence = 0..3;

    for i in 0..3 {assert_eq!(sequence.next(), Some(i))}
}

#[test]
pub fn iter_3() {
    // let mut sequence = 0..3;
    let mut x = 0;
    let sequence = vec![1, 1, 2, 3, 5];
    for i in fibo().take(3) {
        assert_eq!(i, sequence[x]);
        x = x + 1;
    }
}

#[test]
pub fn iter_4() {
    // let mut sequence = 0..3;
    let mut x = 0;
    let sequence = vec![5, 8, 13, 21];
    for i in fibo().skip(4).take(4) {
        assert_eq!(i, sequence[x]);
        x = x + 1;
    }
}