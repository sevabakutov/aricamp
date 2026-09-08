#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Polyline {
    array: Vec<Point>,
}

impl Polyline {
    fn add(&mut self, item: Point) {
        self.array.push(item);
    }

    fn pop(&mut self) -> Point {
        self.array.pop().unwrap_or(Point::default())
    }

    fn get(&self, index: usize) -> Point {
        if self.array.len() > index {
            return self.array[index]
        }
        Point::default()
    }

    fn remove(&mut self, index: usize) -> Point {
        if self.array.len() > index {
            return self.array.remove(index)
        }
        return Point::default()
    }

}

fn main() {
    let a = Point::default();
    let mut b = a;
    b.x = 15;
    b.y = 15;

    let c = Point {x: 30, y: 30};

    let mut arr = Polyline { array: Vec::new() };

    arr.add(a);
    arr.add(b);

    println!("{a:?}, {b:?}, {c:?}, {arr:?}");

    let b = arr.pop();

    arr.add(c);

    println!("b: {b:?}, arr: {arr:?}");


    let a = arr.get(0);
    let d = arr.get(3);

    println!("a: {a:?}, d: {d:?}");
    println!("arr: {arr:?}");


    let f = arr.remove(1);
    println!("f: {f:?}, arr: {arr:?}");
}
