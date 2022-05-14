struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };

    let p3 = Point {
        x: "Hello",
        y: "World",
    };

    let p4 = p1.mixup(p3);

    println!("{}, {}", p4.x, p4.y)
}
