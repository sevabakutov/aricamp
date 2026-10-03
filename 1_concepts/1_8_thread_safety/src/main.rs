use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;struct OnlySync<'a> (MutexGuard<'a, ()>);

struct OnlySend (Cell<u8>);


struct SyncAndSend<T>(Arc<Mutex<T>>);


struct NotSyncNotSend<T>(Rc<T>);



fn main() {
    let sync_and_send = SyncAndSend(Arc::new(Mutex::new(42)));

    let val_clone = SyncAndSend(Arc::clone(&sync_and_send.0));
    thread::spawn(move || {
        let mut guard = val_clone.0.lock().unwrap();
        *guard += 1;
    }).join().unwrap();

    thread::scope(|s| {
        s.spawn(|| {
            let _guard = sync_and_send.0.lock().unwrap();
        });
    });

   let only_send = OnlySend(Cell::new(10));

    thread::spawn(move || {
        only_send.0.set(20);
    }).join().unwrap();


    let dummy_mutex = Mutex::new(());
    let guard = dummy_mutex.lock().unwrap();
    let only_sync = OnlySync(guard);

    thread::scope(|s| {
        s.spawn(|| {
            let _ref = &only_sync;
        });
    });

}
