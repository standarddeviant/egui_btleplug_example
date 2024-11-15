mod async_bridge;
use async_bridge::AsyncBridge;

mod async_msg;
use async_msg::AsyncMsg;

mod async_ble;

mod sync_gui;

use std::time::{Duration, Instant};

// [egui] <--async_bridge--> [btleplug]

fn main() {
    // TODO: replace test logic w/ gui.rs
    //       i.e. - put async_bridge logic, etc.

    // make our async bridge
    let mut ts = AsyncBridge::new();

    // make + send msg
    let m = AsyncMsg::MsgVersion {
        major: 1,
        minor: 2,
        patch: 3,
    };
    ts.send_to_async(m);

    // make + send msg
    let m = AsyncMsg::ScanStart {
        filter: "".into(),
        duration: 5.0 as f32,
    };
    ts.send_to_async(m);

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
