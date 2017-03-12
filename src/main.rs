extern crate threadpool;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

fn main() {
    let t_jobs = 11;
    let t_worker = 3;
    let pool = ThreadPool::new(t_worker);
    let (tx,rx) = channel();

    for i in 0..t_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            let answer = i * i;
            tx.send(answer).unwrap();
        });
    }

    println!("now we begin");

    for _ in 0..t_jobs {
        println!("{}", rx.recv().unwrap());
    }
}
