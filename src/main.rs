fn main() {
    println!("Core Application");

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}