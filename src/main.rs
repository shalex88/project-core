mod tcp_server;
mod command_dispatcher;

fn main() {
    println!("Core Application");

    std::thread::spawn(|| {
        tcp_server::start_server();
    });

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}