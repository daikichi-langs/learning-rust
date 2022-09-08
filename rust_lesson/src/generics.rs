struct Point<T> {
    x: T,
    y: T,
}
struct TypesPoint<T, U> {
    x: T,
    y: U,
}
impl<T, U> TypesPoint<T, U> {
    fn mixup<V, W>(self, other: TypesPoint<V, W>) -> TypesPoint<T, W> {
        TypesPoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let numbers = vec![34, 50, 25, 100, 65];
    //println!("The largest is {}.\r\n", largest_i32(numbers));

    // NOTE: In rust, '' means Char Type, "" means string literal.
    let characters = vec!['a', 'b', 'c', 'd'];
    println!("The largest is {}.\r\n", largest(numbers));
    println!("The largest is {}.\r\n", largest(characters));

    let _p1 = Point { x: 1, y: 2 };
    let _p2 = Point { x: 1.0, y: 2.0 };
    let p3 = TypesPoint { x: 1, y: 2.0 };
    let p4 = TypesPoint { x: "Rust", y: 100 };
    let p5 = p3.mixup(p4);
    println!("p5.x: {}, p5.y: {}", p5.x, p5.y);
}
/* fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];

    // Liner search
    for l in list {
        if largest < l {
            largest = l;
        }
    }
    largest
} */
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    // Liner search
    for l in list {
        if largest < l {
            largest = l;
        }
    }
    largest
}
