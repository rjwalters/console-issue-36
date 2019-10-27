use env_logger;
use log::debug;

fn main() {
    env_logger::init();
    println!("Hello, world!");
    std::thread::spawn(|| {
        debug!("Hello from spawned thread 1");
        debug!("Hello from spawned thread 2");
    });
    debug!("Hello from main thread 1");
    debug!("Hello from main thread 2");
}
