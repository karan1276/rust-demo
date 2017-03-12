extern crate threadpool;

fn main() {
    //println!("Hello, world!");

    use std::sync::mpsc::sync_channel;
    use std::thread;
    use threadpool::ThreadPool;

    let (tx, rx) = sync_channel(0);
    let mut pool = ThreadPool::new_with_name("worker".into(), 2);
    for _ in 0..2 {
        let tx = tx.clone();
        pool.execute(move || {
            let name = thread::current().name().unwrap().to_owned();
            tx.send(name).unwrap();
        });
    }

    for thread_name in rx.iter().take(2) {
        assert_eq!("worker", thread_name);
    }
}
