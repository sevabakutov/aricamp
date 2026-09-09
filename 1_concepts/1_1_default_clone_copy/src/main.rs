#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Polyline {
    array: Vec<Point>,
}

impl Polyline {
    fn push(&mut self, item: Point) {
        self.array.push(item);
    }

    fn pop(&mut self) -> Option<Point> {
        if self.array.len() > 1 {
            return Some(self.array.pop().unwrap())
        }
        None
    }

    fn get(&self, index: usize) -> Option<&Point> {
        if self.array.len() > index {
            return Some(&self.array[index])
        }
        None
    }

    fn remove(&mut self, index: usize) -> Point {
        if self.array.len() > index {
            return self.array.remove(index)
        }
        return Point::default()
    }

    fn new(array: Option<Vec<Point>>) -> Self {
        Polyline {
            array: match array {
                Some(arr) => {
                    if arr.len() >= 1{
                        arr
                    } else {
                        vec![Point::default()]
                    }
                },
                None => vec![Point::default()]
            }
        }
    }

}

impl Default for Polyline {
    fn default() -> Self {
        Self { array: vec![Point::default()] }
    }
}

fn main() {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_default() {
        let point: Point = Default::default();
        assert_eq!(point, Point { x: 0, y: 0 });
    }

    #[test]
    fn test_polyline_push_and_get() {
        let mut polyline = Polyline::new(None);
        let point = Point { x: 5, y: -3 };
        polyline.push(point);

        assert_eq!(polyline.get(0), Some(&Point::default()));
        assert_eq!(polyline.get(1), Some(&point));
        assert_eq!(polyline.get(2), None);
    }

    #[test]
    fn test_polyline_pop() {
        let mut polyline = Polyline::new(None);

        assert_eq!(polyline.pop(), None);
        assert_eq!(polyline.pop(), None);
        assert_eq!(polyline.pop(), None);
    }

    #[test]
    fn test_polyline_clone() {
        let mut polyline = Polyline::new(None);
        polyline.push(Point { x: 1, y: 1 });
        let polyline_clone = polyline.clone();

        assert_eq!(polyline, polyline_clone);

        polyline.push(Point { x: 2, y: 2 });
        assert_ne!(polyline, polyline_clone);
    }
}
