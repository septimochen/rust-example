#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[allow(dead_code)]
fn origin() -> Point {
    Point {
        x: 0.0,
        y: 0.0,
    }
}

#[allow(dead_code)]
fn box_origin() -> Box<Point> {
    Box::new(Point {
        x: 0.0,
        y: 0.0,
    })
}

#[test]
fn std_test1() {
    let point: Point = origin();
    // let rect: Rectangle = Rectangle {
    //     top_left: point,
    //     bottom_right: Point {x: 3.0, y: -4.0}
    // };
    println!("Point occupies {} bytes on the stack", std::mem::size_of_val(&point));
    assert_eq!(std::mem::size_of_val(&point), 16)
}
