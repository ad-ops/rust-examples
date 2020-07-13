use std::thread;
use std::time::Duration;

/// spawned thread stopped when it goes out of scope.
/// This is why it will not complete the count.
fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread: {:?}!", i, thread::current().id());
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread: {:?}!", i, thread::current().id());
        thread::sleep(Duration::from_millis(2));
    }
}