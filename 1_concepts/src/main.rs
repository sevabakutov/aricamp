use std::{fmt::Debug, sync::{Arc, Mutex, Weak}};

#[derive(Debug)]
struct Node<T: Debug> {
    data: T,
    next: Option<Arc<Mutex<Node<T>>>>,
    prev: Option<Weak<Mutex<Node<T>>>>,
}

struct InnerList<T: Debug> {
    start: Option<Arc<Mutex<Node<T>>>>,
    end: Option<Arc<Mutex<Node<T>>>>,
}

struct OuterList<T: Debug>(Mutex<InnerList<T>>);

impl<T: Debug> OuterList<T> {
    fn new() -> Self {
        Self(Mutex::new(InnerList { start: None, end: None}))
    }

    fn push(&self, data: T) {
        let mut inner = self.0.lock().unwrap();

        let pointer = Arc::new(Mutex::new(Node { data: data, next: None, prev: None }));

        if let Some(old_end) = inner.end.as_ref() {
            {
                let mut old_end_locked = old_end.lock().unwrap();
                old_end_locked.next = Some(Arc::clone(&pointer));
            }
            pointer.lock().unwrap().prev = Some(Arc::downgrade(old_end));
        }

        inner.end = Some(Arc::clone(&pointer));

        if inner.start.is_none() {
            inner.start = Some(pointer);
        }

    }

    fn pop(&self) -> Option<T> {
        let mut inner = self.0.lock().unwrap();

        let mut new_end: Option<Arc<Mutex<Node<T>>>> = if let Some(old_end) = inner.end.as_ref() {
            let old_end_mutex = old_end.lock().unwrap();
            if let Some(new) = old_end_mutex.prev.as_ref() {
                new.upgrade()
            } else {
                None
            }
        } else {
            None
        };

        if new_end.is_some() {
            let new_end = new_end.unwrap();
            new_end.lock().unwrap().next = None;
            let old_end_arc = inner.end.take().unwrap();
            let old_end = Arc::try_unwrap(old_end_arc);
            inner.end = Some(new_end);
            return Some(old_end.unwrap().into_inner().unwrap().data);
        } else {
            let return_value = if let Some(n) = inner.end.as_ref() {
                let old_end_arc = inner.end.take().unwrap();
                inner.end = None;
                inner.start = None;
                let old_end = Arc::try_unwrap(old_end_arc);
                Some(old_end.unwrap().into_inner().unwrap().data)
            } else {
                None
            };
                       return return_value;
        }
    }
}


fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn single_thread_test() {
        let outerList: OuterList<u32> = OuterList::new();

        outerList.push(1);
        outerList.push(2);
        outerList.push(3);

        assert_eq!(outerList.pop().unwrap(), 3u32);
        assert_eq!(outerList.pop().unwrap(), 2u32);
        assert_eq!(outerList.pop().unwrap(), 1u32);
        assert_eq!(outerList.pop(), None);

    }

    #[test]
    fn multi_thread_test() {
        let list: Arc<OuterList<u32>> = Arc::new(OuterList::new());

        let l1 = Arc::clone(&list);
        let l2 = Arc::clone(&list);
        let t1 = thread::spawn(move || {
            for data in 1..100{
                l1.push(data);
            }
        });

        let t2  = thread::spawn(move || {
            let mut poped = 0;

            while poped < 99 {
                if l2.pop().is_some() {
                    poped += 1;
                }
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
        assert_eq!(list.pop(), None);


    }
}
