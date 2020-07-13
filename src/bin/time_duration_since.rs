use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut last_instant = Instant::now();
    for i in 1..10 {
        let current_instant = Instant::now();
        let duration_since_last_instance = current_instant.duration_since(last_instant);
        if duration_since_last_instance > Duration::from_millis(5) {
            println!("Bonus! It has been {:?}", duration_since_last_instance);
            last_instant = current_instant;
        }
        println!("{:?}: {}!", duration_since_last_instance, i);
        thread::sleep(Duration::from_millis(2));
    }
}