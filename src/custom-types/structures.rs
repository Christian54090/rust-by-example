#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// a unit struct
struct Nil;

// a tuple struct
struct Pair(i32, f32);

// a struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let width = self.p1.x + self.p2.x;
        let height = self.p1.y + self.p2.y;
        width * height
    }

    fn is_square(&self) -> bool {
        self.width() + self.height() < 0.01
    }

    fn width(&self) -> f32 {
        self.p1.x - self.p2.x
    }

    fn height(&self) -> f32 {
        self.p1.y - self.p2.y
    }
}

fn square(point: Point, size: f32) -> Rectangle {
    let new_x = point.x + size;
    let new_y = point.y + size;
    Rectangle { p1: point, p2: Point { x: new_x, y: new_y } }
}

fn main() {
    let peter = Person { name: "Peter", age: 27 };
    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let new_point = Point { x: 0.1, ..point }; // new_point.y == point.y
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // destructure the point the point using a 'let' binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // instantiate a unit struct
    let _nil = Nil;

    // instantiate a tuple struct
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // destructure the tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle {
        p1: Point { x: 3.4, y: 1.3 },
        p2: Point { x: 7.8, y: 5.0 },
    };
    println!("the area of rect is {}", rect.area());

    let square = square(Point{ x: 1.1, y: 2.2}, 3.7);
    println!("is this object square? {}", square.is_square());
}