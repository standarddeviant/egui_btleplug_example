mod async_bridge;
use async_bridge::AsyncBridge;

mod async_msg;
use async_msg::AsyncMsg;

mod async_ble;

mod sync_gui;
use sync_gui::GuiApp;

use std::time::{Duration, Instant};

// sync_gui: {app: egui, async_bridge: ...}
// sync_gui: {app: egui, async_bridge: ...}

// data_flow: {[egui] <--async_bridge--> [btleplug]

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    let _eframe_result = eframe::run_native(
        "egui_btleplug_example",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(GuiApp::new(cc))) // creation context
        }),
    );
}

fn test_old_main() {
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
