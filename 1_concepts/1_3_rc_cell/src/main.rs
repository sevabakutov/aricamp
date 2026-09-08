use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug)]
struct GlobalStack<T: Clone> {
    stack: Rc<RefCell<Vec<T>>>,
}

impl<T: Clone> GlobalStack<T> {
    fn pop(&self) -> T{
        self.stack.borrow_mut().pop().unwrap()
    }

    fn push(&self, item: T) {
        self.stack.borrow_mut().push(item);
    }

    fn get_last(&self) -> T {
        let a = self.stack.borrow();
        a[a.len() - 1].clone()
    }

    fn new(stack: Vec<T>) -> GlobalStack<T>{
        GlobalStack { stack: Rc::new(RefCell::new(stack)) }
    }
}

impl<T: Clone> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        let new_stack = self.stack.clone();
        GlobalStack { stack: new_stack }
    }
}

fn main() {
    let stack = GlobalStack::new(vec![1,2,3]);

    println!("{stack:?}");

    stack.push(4);

    println!("{stack:?}");


    let a = stack.pop();
    println!("{stack:?}, {a}");

    let a = stack.get_last();
    println!("{stack:?}, {a}");

    let stack2 = stack.clone();

    println!("{stack:?}, {stack2:?}");

    stack2.push(4);
    stack.push(5);
    println!("{stack:?}, {stack2:?}");

}

