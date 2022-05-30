use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // cloning tx for the second thread to create multiple transmitters
    let tx2 = tx.clone();

    // create another thread that will send a message to main thread
    thread::spawn(move || {
        // let msg = String::from("hi");
        // tx.send(msg).unwrap();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the "),
            String::from("thread1"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // create another thread that will send a message to main thread
    thread::spawn(move || {
        // let msg = String::from("hi");
        // tx.send(msg).unwrap();

        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // .rec() will block the main channel until a message is received
    // .try_recv() will not block the channel and will return a result type immediately
    // let received = rx.recv().unwrap();

    for received in rx {
        println!("Got :{}", received);
    }
}
