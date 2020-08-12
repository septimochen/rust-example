#[derive(Clone, Copy)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
pub fn run() {
    let c = 'Q';

    let ref ref_c1 = c;
    let rec_c2 = &c;

    println!("ref_c1 equals ref_c2: {:#?}", *ref_c1 == *rec_c2);
    assert_eq!(*rec_c2, *ref_c1);
}

#[test]
pub fn run_ref_2() {
    let point = Point { x: 0, y: 0 };
    let _copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };
    assert_eq!(_copy_of_x, 0);
}

#[test]
pub fn run_ref_3() {
    let point = Point { x: 0, y: 0 };
    let mut mut_point = point;
    let Point {
        x: _,
        y: ref mut mut_ref_to_y,
    } = mut_point;
    *mut_ref_to_y = 1;

    assert_eq!(mut_point.y, 1);
}

#[test]
pub fn run_ref_4() {
    let mut mut_tuple = (Box::new(5u32), 3u32);
    let (_, ref mut x) = mut_tuple;
    *x = 2u32;
    println!("{:?}", mut_tuple);
    assert_eq!(2, mut_tuple.1);
}
