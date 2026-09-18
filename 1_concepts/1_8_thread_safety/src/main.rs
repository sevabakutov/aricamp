
use std::{cell::Cell, rc::Rc};

struct OnlySync (*mut u8);

unsafe impl Sync for OnlySync { }

struct OnlySend<T> {
    a: Cell<T>
}

unsafe impl<T> Send for OnlySend<T> {}

struct SyncAndSend ();

unsafe impl Sync for SyncAndSend {}
unsafe impl Send for SyncAndSend {}

struct NotSyncNotSend<T> {
    a: Rc<T>
}



fn main() {
}
