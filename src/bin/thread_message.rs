use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // receiver can only be one and have to be on main thread.
    let tx2 = tx.clone(); // need a new sender since they cannot be shared between different threads.

    thread::spawn(move || {
        for i in 1..5 {
            tx.send(i).unwrap();
            println!("{:?} Send: {}", thread::current().id(), i);
        }
    });
    
    thread::spawn(move || {
        for i in 5..10 {
            tx2.send(i).unwrap();
            println!("{:?} Send: {}", thread::current().id(), i);
        }
    });

    println!("Sending messages:");
    loop {
        match rx.recv() {
            Ok(v) => println!("{:?} Got: {}", thread::current().id(), v),
            Err(e) => {
                println!("{:?} Exiting: {}", thread::current().id(), e);
                break;
            }
        };
    }
}
