use rand::RngExt;
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {

    let (tx, rx) = mpsc::sync_channel::<Vec<u8>>(2);
    let sync_rx = Arc::new(Mutex::new(rx));

    thread::spawn(move || {
            let mut rng = rand::rng();

            loop {
                let mut matrix = vec![0u8; 4096];

                rng.fill(&mut matrix[..]);

                if tx.send(matrix).is_err() {
                    break;
                }
            }
    });

    for i in 0..2 {
        let cloned_rx = sync_rx.clone();

        thread::spawn(move || {

            loop {
                let matrix = {
                    let rx = cloned_rx.lock().unwrap();
                    match rx.recv() {
                        Ok(m) => m,
                        Err(_) => break,
                    }
                };


                let total_sum: u64 = thread::scope(|s| {
                    let mid = matrix.len() / 2;
                    let (first_half, second_half) = matrix.split_at(mid);

                    let handle = s.spawn(move || {
                        first_half.iter().map(|&x| x as u64).sum::<u64>()
                    });

                    let second_sum: u64 = second_half.iter().map(|&x| x as u64).sum::<u64>();

                    let first_sum = handle.join().unwrap();
                    first_sum + second_sum
                });

                println!("thread: {}, sum: {}",i, total_sum);
            }

        });
    }
    thread::sleep(Duration::from_secs(3));
}
