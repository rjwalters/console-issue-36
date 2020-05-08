use console::Term;
use env_logger;

fn main() {
    env_logger::init();
    println!("Hello, world!");
    let thread = std::thread::spawn(|| {
        let term = Term::stdout();
        let key = term.read_key();
        print!("Please press any button\n");
        print!("{:?}\n", key.unwrap());
        print!("Hello from spawned thread 1\n");
        print!("Hello from spawned thread 2\n");
    });

    print!("Hello from main thread 1\n");
    print!("Hello from main thread 2\n");

    thread.join().unwrap();
}
