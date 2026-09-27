use std::{sync::{Arc, Mutex, Weak}};

struct Node<T> {
    data: T,
    next: Option<Arc<Mutex<Node<T>>>>,
    previous: Option<Weak<Mutex<Node<T>>>>,
}

struct DoubleLinkedList<T> {
    start: Arc<Mutex<Node<T>>>,
    end: Arc<Mutex<Node<T>>>,
    size: usize,
}

impl<T> DoubleLinkedList<T>
where
    T: Clone
{
    fn new(data: T) -> Self {
        let a = Arc::new(Mutex::new(
                Node { data, next: None, previous: None }
        ));
        DoubleLinkedList { start: a.clone(), end: a.clone(), size: 1usize }
    }

    fn push(&mut self, data: T) {
        let new_node = Arc::new(Mutex::new(Node {
            data,
            next: None,
            previous: None,
        }));

        new_node.lock().unwrap().previous = Some(Arc::downgrade(&self.end));
        self.end.lock().unwrap().next = Some(new_node.clone());
        self.end = new_node;
        self.size += 1;
    }

    fn add(&mut self, data: T) {
        let new_node = Arc::new(Mutex::new( Node {
            data,
            next: None,
            previous: None,
        }));

        {
            new_node.lock().unwrap().next = Some(self.start.clone());
            self.start.lock().unwrap().previous = Some(Arc::downgrade(&new_node.clone()));
            self.size += 1;
        }

        self.start = new_node.clone();
    }

    fn pop(&mut self) -> Option<T> {
        if self.size <= 1 {
            return None;
        }

        let old_end_arc = self.end.clone();

        let (new_data, new_end) = {
            let old_end = old_end_arc.lock().unwrap();
            let prev_weak = old_end.previous.as_ref().unwrap();
            let new_end = prev_weak.upgrade().unwrap();
            (old_end.data.clone(), new_end)
        };

        new_end.lock().unwrap().next = None;
        self.end = new_end;
        self.size -= 1;

        Some(new_data)
    }

    fn del_first(&mut self) -> Option<T> {
        if self.size <= 1 {
            return None;
        }

        let old_start_arc = self.start.clone();

        let (new_data, next_node) = {
            let old_start = old_start_arc.lock().unwrap();
            let next_node = old_start.next.clone().unwrap();
            (old_start.data.clone(), next_node)
        };

        next_node.lock().unwrap().previous = None;
        self.start = next_node;
        self.size -= 1;

        Some(new_data)
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut list = DoubleLinkedList::new(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.size, 3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.size, 1);

        assert_eq!(list.pop(), None);
        assert_eq!(list.size, 1);
    }

    #[test]
    fn test_add_and_del_first() {
        let mut list = DoubleLinkedList::new(10);
        list.add(20);
        list.add(30);

        assert_eq!(list.size, 3);
        assert_eq!(list.del_first(), Some(30));
        assert_eq!(list.del_first(), Some(20));
        assert_eq!(list.size, 1);

        assert_eq!(list.del_first(), None);
        assert_eq!(list.size, 1);
    }

    #[test]
    fn test_everything() {
        let mut list = DoubleLinkedList::new(100);
        list.push(200);
        list.add(50);

        assert_eq!(list.size, 3);
        assert_eq!(list.del_first(), Some(50));
        assert_eq!(list.pop(), Some(200));
        assert_eq!(list.size, 1);
    }
}
