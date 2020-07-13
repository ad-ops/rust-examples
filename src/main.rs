fn main() {
    println!("Try running each binary under /bin...");

    let duration = std::time::Duration::from_millis(1000);
    print!("\ra");
    std::thread::sleep(duration);
    print!("\rb");
    std::thread::sleep(duration);
    print!("\rc");
    std::thread::sleep(duration);
    print!("\rd");
    std::thread::sleep(duration);
    print!("\re");
}
