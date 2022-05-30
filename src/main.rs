use std::{thread, time::Duration};

fn main() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("The number from spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // this will block the main thread from finishing until handler gets finished
    // when main thread finishes, all the other threads finish
    // the position where join is called can effect the running of the process
    handler.join().unwrap();

    for i in 1..5 {
        println!("The number from main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    // closure may outlive the current function, but it borrows `v`, which is owned by the current function
    // may outlive borrowed value `v`
    // hence we move the ownership of v to the closure using 'move'
    let handler = thread::spawn(move || {
        println!("{:?}", v);
    });

    handler.join().unwrap();
}
