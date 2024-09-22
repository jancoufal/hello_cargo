use std::thread;
use std::time::Duration;

fn main() {
    let counting_thread = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_secs_f32(1.33));
    }

    // counting_thread.join().unwrap();
}