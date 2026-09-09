use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct GlobalStack<T> {
    stack: Rc<RefCell<Vec<T>>>,
}

impl<T> GlobalStack<T> {
    fn pop(&self) -> Option<T>{
        let mut stack = self.stack.borrow_mut();

        if stack.len() > 0 {
            Some(stack.pop().unwrap())
        } else {
            None
        }
    }

    fn push(&self, item: T) {
        self.stack.borrow_mut().push(item);
    }
    fn new(stack: Option<Vec<T>>) -> GlobalStack<T>{
        match stack {
            None => GlobalStack { stack: Rc::new(RefCell::new(Vec::new())) },
            Some(st) => GlobalStack { stack: Rc::new(RefCell::new(st)) }
        }
    }
}

impl<T: Clone> GlobalStack<T> {
    fn get_last(&self) -> Option<T> {
        let a = self.stack.borrow();
        if a.len() > 0 {
            Some(a[a.len() - 1].clone())
        } else{
            None
        }
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        Self { stack: Rc::clone(&self.stack) }
    }
}

fn main() {
}


#[cfg(test)]
mod test {
    use std::alloc::GlobalAlloc;

use super::*;

    #[test]
    fn test_new() {
        let stack1: GlobalStack<i32> = GlobalStack::new(None);
        let stack2 = GlobalStack { stack: Rc::new(RefCell::new(Vec::new())) };
        let stack3 = GlobalStack { stack: Rc::new(RefCell::new(vec![1,2,3]))};
        let stack4 = GlobalStack::new(Some(vec![1,2,3]));
        assert_eq!(stack2, stack1);
        assert_eq!(stack3, stack4);
    }

    #[test]
    fn test_push() {
        let stack1: GlobalStack<i32> = GlobalStack::new(None);
        stack1.push(1);
        stack1.push(2);
        let stack2 = GlobalStack::new(Some(vec![1,2]));

        assert_eq!(stack1, stack2);

    }

    #[test]
    fn test_pop() {
        let stack1: GlobalStack<i32> = GlobalStack::new(None);
        assert_eq!(None, stack1.pop());
        let stack2 = GlobalStack::new(Some(vec![1,2,3]));
        assert_eq!(stack2.pop(), Some(3));
        assert_eq!(stack2.pop(), Some(2));
        assert_eq!(stack2.pop(), Some(1));
    }

    #[test]
    fn test_clone(){
        let stack1 = GlobalStack::new(Some(vec![1,2,3]));
        let stack2 = stack1.clone();
        assert_eq!(stack1.pop(), Some(3));
        assert_eq!(stack2.pop(), Some(2));
        assert_eq!(stack1.pop(), Some(1));
        assert_eq!(stack2.pop(), None);
    }

    #[test]
    fn test_get_last() {
        let stack = GlobalStack::new(Some(vec![1,2,3]));
        assert_eq!(stack.get_last(), Some(3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.get_last(), Some(2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.get_last(), Some(1));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.get_last(), None);
        assert_eq!(stack.pop(), None);
    }
}
