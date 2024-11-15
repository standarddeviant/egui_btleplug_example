use crate::async_msg::AsyncMsg;
use crate::async_msg::GenericResult::*;

use crate::async_bridge::AsyncBridge;

use eframe;
// use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::sync::mpsc::Sender;

enum BLEState {
    Disconnected,
    Scanning,
    Connecting,
    Connected,
}

// use crate::ble_gatt;
// use prost::Message;

// use anyhow::Result;

// use proble::messages::ProbleMessage;
// use proble::messages::ScanStart;
// use proble::messages::proble_message::Msg;

// use crate::{ProbleMessage};
// use crate::msg::{ProbleMessage, ProbleOneof, ScanStart};
// , ProbleOneof
// use crate::msg::ProbleMessage;

// use proble::messages::ProbleMessage;
// use chrono::Instant;
// use tokio::sync::mpsc::{channel, Sender, Receiver};

// use crate::messages;

// use crate::messages;

// use crate::messages;

// enum BLEState {
//     Disconnected,
//     Scanning,
//     Connecting,
//     Connected,
// }

// fn foo() -> anyhow::Result<()> {
//     if errored {
//         bail!(MyError::MyVariant { actual: 0, expected: 1 })
//     }
// }
// enum BLEMessage {
//     ScanStart(f32),
//     ScanStop,
//     ScanResults(Vec<Peripheral>),
//     ConnectStart(Peripheral),
//     ConnectResult,
// }

// ProbleMessage
// use create::mProbleMessage::

// We derive Deserialize/Serialize so we can persist app state on shutdown.

// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GuiApp {
    // #[serde(skip)] // This how you opt-out of serialization of a field
    ble_state: BLEState,

    // Example stuff:
    last_address: String,

    // Example stuff:
    last_name: String,

    // #[serde(skip)] // This how you opt-out of serialization of a field
    pstrings: Vec<String>,

    // let (to_dev_tx, to_dev_rx) = tokio::sync::mpsc::new(32);
    // let (from_dev_tx, from_dev_rx) = tokio::sync::mpsc::new(32);
    // #[serde(skip)] // This how you opt-out of serialization of a field
    // to_ble_send: tokio::sync::mpsc::Sender<AsyncMsg>,
    // // #[serde(skip)] // This how you opt-out of serialization of a field
    // to_app_recv: tokio::sync::mpsc::Receiver<AsyncMsg>,

    // #[serde(skip)] // This how you opt-out of serialization of a field
    scan_start_time: Instant,

    scan_duration: Duration,
    // #[serde(skip)] // This how you opt-out of serialization of a field
    // rt_handle: tokio::runtime::Handle,
    bridge: AsyncBridge,
}

// impl Default for ProbleApp {
//     fn default() -> Self {
//         let (junk_send, junk_recv) = tokio::sync::mpsc::channel(32);
//         let junk_rt = tokio::runtime::Runtime::new().unwrap();
//         let junk_rt_handle = junk_rt.handle();
//
//         // let (_, junk_recv) = std::sync::mpsc::channel();
//         Self {
//             // Example stuff:
//             ble_state: BLEState::Disconnected,
//             last_address: "".to_owned(),
//             last_name: "".to_owned(),
//             pstrings: [].into(),
//             to_ble_send: junk_send,
//             to_app_recv: junk_recv,
//             scan_start: Instant::now(),
//             scan_duration: Duration::from_secs_f32(0.0 as f32),
//             // rt_handle: junk_rt_handle.clone(),
//         }
//     }
// }

async fn test_transport_fn(
    to_app_send: tokio::sync::mpsc::Sender<AsyncMsg>,
    mut to_ble_recv: tokio::sync::mpsc::Receiver<AsyncMsg>,
) {
    // recv_from_app
    // info!("DBG: info test!");
    println!("DBG: test_transport_fn");
    println!("DBG: test_transport_fn: waiting on to_ble_recv.recv().await ... ");

    match to_ble_recv.recv().await {
        Some(m) => {
            println!("DBG: test_transport_fn: m = {m:?}");
        }
        None => {
            println!("DBG: test_transport_fn: m = None...");
            // return early?
        }
    }

    let sr = AsyncMsg::ScanResult {
        result: ResultSuccess.into(),
        periphs: vec![],
    };
    println!("\nDBG: test_transport_fn: made msg: {sr:#?}");
    match to_app_send.send(sr).await {
        Ok(_good) => {
            println!("DBG: test_transport_fn: to_app_send.send: SUCESS!");
        }
        Err(_bad) => {
            eprintln!("ERR: test_transport_fn: can't use");
            eprintln!("    to_app_send.send(sr).await");
            eprintln!("    {_bad}");
        }
    }
}

impl GuiApp {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        // to_ble_send: tokio::sync::mpsc::Sender<ProbleMsg>,
        // to_app_recv: tokio::sync::mpsc::Receiver<ProbleMsg>,
    ) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        // let (to_ble_send, to_ble_recv) = tokio::sync::mpsc::channel(32);
        // let (to_app_send, to_app_recv) = tokio::sync::mpsc::channel(32);

        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        // spawn a tokio runtime/btleplug task
        // std::thread::spawn(move || {
        //     let rt: Runtime = Builder::new_current_thread().enable_all().build().unwrap();
        //     rt.block_on(async move {
        //         test_transport_fn(to_app_send, to_ble_recv).await;
        //     });
        // });

        // let (junk_send, _) = tokio::sync::mpsc::channel(32);
        // let (_, junk_recv) = std::sync::mpsc::channel();

        // Default::default()
        GuiApp {
            ble_state: BLEState::Disconnected,
            last_address: "".into(),
            last_name: "".into(),
            pstrings: vec![],
            scan_start_time: Instant::now(),
            scan_duration: Duration::from_secs_f32(5.0),
            bridge: AsyncBridge::new(),
            // rt_handle: rt.handle().clone(),
        }
    }
}

impl eframe::App for GuiApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.request_repaint();
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // println!(
            //     "DBG: to_app_recv.is_channel_closed() = {}",
            //     self.to_app_recv.is_closed()
            // );
            match self.ble_state {
                BLEState::Disconnected => {
                    if ui.button("Start Scan").clicked() {
                        self.ble_state = BLEState::Scanning;
                        self.scan_start_time = Instant::now();

                        self.bridge.send_to_async(AsyncMsg::ScanStart {
                            filter: "".into(),
                            duration: 5.0,
                        });
                    }
                }
                BLEState::Scanning => {
                    // let since = Instant::now() - self.scan_start;
                    ui.label(format!("Scanning...")); // time is {:#?} seconds", since as u32));

                    match self.bridge.try_recv_from_async() {
                        Some(AsyncMsg::ScanResult { result, periphs }) => {
                            println!("sync_gui: got ScanResult ({result:?})");
                            println!("    {periphs:?}");
                            // self.bridge.send_to_async(AsyncMsg::ConnectStart {
                            //     ???
                            // });
                            self.bridge.send_to_async(AsyncMsg::MsgVersion {
                                major: 1,
                                minor: 2,
                                patch: 3,
                            });
                        }
                        Some(unhandled) => {
                            eprintln!("sync_gui: got (UNHANDLED) {unhandled:?}");
                        }
                        None => {
                            // eprintln!("to_app_recv: got None...");
                            // nothing to do
                        }
                    }
                    // match self.to_app_recv.try_recv() {
                    //     Ok(m) => {
                    //         println!("app: to_app_recv = ...");
                    //         println!("{m:?}");
                    //     }
                    //     Err(bad) => {
                    //         eprintln!("hmmm... {bad}");
                    //     }
                    // }
                }
                BLEState::Connecting => {}
                BLEState::Connected => {}
            };
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     self.value += 1.0;
            // }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
