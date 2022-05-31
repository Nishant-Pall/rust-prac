use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // let m = Mutex::new(5);
    // {
    //     // locking the m variable
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("{:?}", m);

    let counter = Arc::new(Mutex::new(0));

    let mut handler = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handler.push(handle);
    }

    // block main thread until handle is finished
    for handle in handler {
        handle.join().unwrap()
    }

    println!("{}", counter.lock().unwrap())
}
