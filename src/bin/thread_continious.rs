use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread: {:?}!", i, thread::current().id());
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread: {:?}!", i, thread::current().id());
            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        println!("hi from the main thread: {:?}!", thread::current().id());
        thread::sleep(Duration::from_secs(3));
    }
}