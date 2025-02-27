// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.
// This program should wait until all the spawned threads have finished before exiting.


use std::thread::{self, JoinHandle};
use std::time::Duration;


fn main() {

    let mut handles: Vec<JoinHandle<()>> = vec![];
    for i in 0..10 {
        let join_handle_struct: JoinHandle<()> = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
        });

        handles.push(join_handle_struct);
    }

    let mut completed_threads = 0;
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        completed_threads += 1;
    }

    if completed_threads != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
    
}
