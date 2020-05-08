use console::Term;
use env_logger;
use log::debug;

fn main() {
    env_logger::init();
    println!("Hello, world!");
    let thread = std::thread::spawn(|| {
        let term = Term::stdout();
        let key = term.read_key();
        debug!("Please press any button\n");
        print!("{:?}\n", key.unwrap());
        debug!("Hello from spawned thread 1\n");
        debug!("Hello from spawned thread 2\n");
    });

    debug!("Hello from main thread 1\n");
    debug!("Hello from main thread 2\n");

    thread.join().unwrap();
}
