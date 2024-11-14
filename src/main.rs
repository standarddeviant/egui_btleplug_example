mod async_bridge;
use async_bridge::{Task, TaskBridge};

use std::time::{Duration, Instant};

// [egui] <--async_bridge--> [btleplug]

fn main() {
    println!("Hello, world!");

    // make our async bridge
    let mut ts = TaskBridge::new();

    let t = Task::new("yolo".into());
    // use our async bridge - would be used from egui context
    ts.send_to_async(t);

    let start = Instant::now();
    loop {
        match ts.try_recv_from_async() {
            Some(resp) => {
                println!("Got response {resp:?}");
                break;
            }
            None => (),
        }
        if Instant::now() - start > Duration::from_secs_f32(5.0) {
            break;
        }
    }

    println!("Done with main");
}
