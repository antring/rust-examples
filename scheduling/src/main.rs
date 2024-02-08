use clokwerk::{Job, Scheduler, TimeUnits};
use std::thread;
use std::time::Duration;
use clokwerk::Interval::Thursday;

fn test_function() {
    println!("Hello from a function");
}

fn main() {
    println!("Starting");

    let mut scheduler = Scheduler::new();

    scheduler.every(2.seconds()).run(|| println!("Hello every 2 seconds"));
    scheduler.every(3.seconds()).run(test_function);
    scheduler.every(Thursday).at("22:13:50").run(|| println!("How about this?"));

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10));
    }
}
