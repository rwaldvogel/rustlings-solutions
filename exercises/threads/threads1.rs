// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    for i in 0..10 {
        let mut status_shared = status.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(250+200*i));
            let mut local_status = status_shared.lock().unwrap();
            local_status.jobs_completed += 1;
            println!("done")
        });
    }
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
