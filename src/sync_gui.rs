use crate::async_msg::{AsyncMsg, BLEOperation};

use crate::async_msg::GenericResult::*;

use crate::async_bridge::AsyncBridge;

use crate::ble_constants::get_default_char_descs;

use btleplug::api::CharPropFlags;
use btleplug::api::{Characteristic, PeripheralProperties};
use eframe;
use egui::ahash::HashSet;
// use egui::ahash::RandomState;
use egui_extras::{Column, TableBuilder};
use std::collections::hash_map::RandomState;
// use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use uuid::Uuid;

enum BLEState {
    Disconnected,
    Scanning,
    Connectable,
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

    /// simple scan filter
    filter: String,

    // #[serde(skip)] // This how you opt-out of serialization of a field
    props_vec: Vec<(usize, PeripheralProperties)>,

    connected_index: usize,
    connected_props: PeripheralProperties,

    // #[serde(skip)] // This how you opt-out of serialization of a field
    chars: BTreeSet<Characteristic>,

    svc_keys: Vec<Uuid>,
    svc_map: HashMap<Uuid, Vec<Characteristic>, RandomState>,

    waiting_payload: Option<AsyncMsg>,

    char_values: HashMap<Uuid, Vec<u8>>,

    char_descs: HashMap<Uuid, String>,

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

fn get_props_desc(flags: CharPropFlags) -> String {
    let mut out: Vec<String> = vec![];
    if flags.contains(CharPropFlags::READ) {
        out.push("Rd".into());
    }
    if flags.contains(CharPropFlags::WRITE) {
        out.push("Wr".into());
    }
    if flags.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE) {
        out.push("Wr(w/o)".into());
    }
    if flags.contains(CharPropFlags::NOTIFY) {
        out.push("Notif".into());
    }
    if flags.contains(CharPropFlags::INDICATE) {
        out.push("Indicate".into());
    }

    out.join("/")
}

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
        props_vec: vec![],
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

fn periph_desc_string(props: &PeripheralProperties) -> String {
    let mut out: Vec<String> = vec![];
    if let Some(name) = &props.local_name {
        out.push(format!("name={}", name));
    }

    if let Some(rssi_val) = &props.rssi {
        out.push(format!("rssi={}", rssi_val));
    }

    out.push(format!("{:?}", props.address));

    if props.manufacturer_data.len() > 0 {
        out.push(format!("{:?}", props.manufacturer_data));
    }

    out.join(" : ")
}

fn sort_svcs_chars(
    chars: BTreeSet<Characteristic>,
) -> (Vec<Uuid>, HashMap<Uuid, Vec<Characteristic>, RandomState>) {
    let svc_uuid_set: std::collections::HashSet<Uuid, egui::ahash::RandomState> =
        HashSet::from_iter(chars.clone().iter().map(|c| c.service_uuid));
    let mut svc_uuid_vec: Vec<Uuid> = Vec::from_iter(svc_uuid_set.iter().map(|r| r.clone()));
    // INFO: making the output type not use references is helpful to not worry about lifetimes in structs...
    // INFO: so we make a vector of reference clones for convenience
    // let svc_uuid_vec: Vec<Uuid> = Vec::from_iter(svc_uuid_set.iter());
    svc_uuid_vec.sort();

    // INFO: making the output type not use references is helpful to not worry about lifetimes in structs...
    // INFO: so we make a vector of reference clones for convenience
    let tmpchars: Vec<Characteristic> = chars.into_iter().map(|f| f.clone()).collect();

    let mut svc_map: HashMap<Uuid, Vec<Characteristic>, RandomState> =
        HashMap::<Uuid, Vec<Characteristic>, RandomState>::new();
    for svc_uuid in svc_uuid_vec.clone() {
        let mut tmpvec: Vec<Characteristic> = tmpchars
            .clone()
            .iter()
            .map(|r| r.clone())
            .filter(|c| c.service_uuid == svc_uuid)
            .collect();
        tmpvec.sort();
        svc_map.insert(svc_uuid, tmpvec);
    }

    (svc_uuid_vec, svc_map)
}

impl GuiApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Default::default()
        GuiApp {
            ble_state: BLEState::Disconnected,
            filter: "".into(),
            props_vec: vec![],
            connected_index: 1_000_000,
            connected_props: PeripheralProperties::default(),
            chars: BTreeSet::new(),
            svc_keys: vec![],
            svc_map: HashMap::<Uuid, Vec<Characteristic>, RandomState>::from_iter(
                std::iter::empty(),
            ),
            waiting_payload: None,
            char_values: HashMap::<Uuid, Vec<u8>, RandomState>::from_iter(std::iter::empty()),
            char_descs: get_default_char_descs(),
            scan_start_time: Instant::now(),
            scan_duration: Duration::from_secs_f32(5.0),
            bridge: AsyncBridge::new(),
            // rt_handle: rt.handle().clone(),
        }
    }

    pub fn draw_top_panel(&mut self, ctx: &egui::Context) {
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
    } // NOTE: end: draw_top_panel

    pub fn draw_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.draw_central_panel_left(ctx, ui);
            });
        });
    }

    pub fn draw_central_panel_left(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        match self.ble_state {
            BLEState::Disconnected => {
                ui.horizontal(|ui| {
                    if ui.button("Start Scan").clicked() {
                        self.ble_state = BLEState::Scanning;
                        self.scan_start_time = Instant::now();

                        self.bridge.send_to_async(AsyncMsg::ScanStart {
                            filter: "".into(),
                            duration: 5.0,
                        });
                    }

                    let filter_label = ui.label("Filter: ");
                    ui.text_edit_singleline(&mut self.filter)
                        .labelled_by(filter_label.id);
                });
            }

            BLEState::Scanning => {
                // let since = Instant::now() - self.scan_start;
                ui.label(format!("Scanning...")); // time is {:#?} seconds", since as u32));

                match self.bridge.try_recv_from_async() {
                    Some(AsyncMsg::ScanResult { result, props_vec }) => {
                        println!("sync_gui: got ScanResult ({result:?})");
                        self.props_vec = props_vec;
                        self.ble_state = BLEState::Connectable;
                    }
                    Some(unhandled) => {
                        eprintln!("sync_gui: got (UNHANDLED) {unhandled:?}");
                    }
                    None => {
                        // eprintln!("to_app_recv: got None...");
                        // nothing to do
                    }
                }
            }

            BLEState::Connectable => {
                ui.horizontal(|ui| {
                    if ui.button(format!("Rescan")).clicked() {
                        self.ble_state = BLEState::Scanning;
                        self.bridge.send_to_async(AsyncMsg::ScanStart {
                            filter: self.filter.clone(),
                            duration: self.scan_duration.as_secs_f32(),
                        });
                    }
                    let filter_label = ui.label("Filter: ");
                    ui.text_edit_singleline(&mut self.filter)
                        .labelled_by(filter_label.id);
                });

                ui.label(format!("Choose a peripheral to connect..."));
                for (ix, p) in &self.props_vec {
                    let pdesc = periph_desc_string(&p);
                    if !pdesc.contains(self.filter.as_str()) {
                        continue;
                    }
                    if ui
                        .button(format!("Connect: {}", periph_desc_string(&p)))
                        .clicked()
                    {
                        self.bridge.send_to_async(AsyncMsg::ConnectStart {
                            index: *ix,
                            props: p.clone(),
                        });
                        self.ble_state = BLEState::Connecting;
                        break;
                    };
                }
            }

            BLEState::Connecting => {
                ui.label("Connecting...");
                if ui.button(format!("Rescan")).clicked() {
                    self.ble_state = BLEState::Scanning;
                    self.bridge.send_to_async(AsyncMsg::ScanStart {
                        filter: "".into(),
                        duration: 5.0,
                    });
                }

                match self.bridge.try_recv_from_async() {
                    Some(AsyncMsg::ConnectResult {
                        result,
                        index,
                        props,
                    }) => {
                        match result {
                            ResultSuccess => {
                                println!("sync_gui: connected");
                            }
                            _ => {
                                println!("sync_gui: got {result:?}");
                                self.ble_state = BLEState::Disconnected;
                            }
                        }
                        // we can't do much until the services are discovered, so just chill
                        // here for now?
                    }
                    Some(AsyncMsg::Characteristics { chars }) => {
                        println!("sync_gui: received ({}) chars", chars.len());
                        self.chars = chars.clone();
                        self.ble_state = BLEState::Connected;

                        (self.svc_keys, self.svc_map) = sort_svcs_chars(chars.clone());

                        // construct a sorted tree repr of [services --> chars] exactly (1) time so the display is consistent
                        for svc_uuid in self.svc_keys.clone() {
                            ui.collapsing(format!("Service: {svc_uuid:?}"), |ui| {
                                let cvec =
                                    self.svc_map.get(&svc_uuid).expect("Key error in svc_map");
                                for c in cvec {
                                    ui.label(format!("    {c:?}"));
                                }
                            });
                        }
                    }
                    Some(unhandled) => {
                        println!("sync_gui: got (UNHANDLED) msg: {unhandled:?}");
                    }
                    None => {}
                }
            }

            BLEState::Connected => {
                if ui.button(format!("Disconnect")).clicked() {
                    self.ble_state = BLEState::Disconnected;
                    self.bridge.send_to_async(AsyncMsg::DisconnectStart {
                        index: self.connected_index,
                        props: self.connected_props.clone(),
                    });
                }

                // TODO: receiving values like this could be done outside this nested logic...
                // TODO: move this logic to a common helper function
                match self.bridge.try_recv_from_async() {
                    Some(AsyncMsg::Payload { payload, char, op }) => {
                        if let Some(_some_waiting_payload) = self.waiting_payload.clone() {
                            // TODO: check the char+op agains self.waiting_payload
                            self.waiting_payload = None;
                            self.char_values.insert(char.uuid, payload);
                        }
                    }
                    Some(unhandled) => {
                        eprintln!("sync_gui: got (UNHANDLED) msg: {unhandled:?}");
                    }
                    None => (),
                }

                for svc_uuid in self.svc_keys.clone() {
                    self.draw_svc_table(ctx, ui, svc_uuid);
                }
            } // NOTE: end BLEState::Connected
        }
        ui.separator();
    }
    pub fn draw_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    pub fn draw_char_buttons(&mut self, ui: &mut egui::Ui, c: Characteristic) {
        ui.horizontal(|ui| {
            if c.properties.contains(CharPropFlags::READ) {
                if ui.button("Read").clicked() {
                    let m = AsyncMsg::Payload {
                        payload: vec![],
                        char: c.clone(),
                        op: BLEOperation::Read,
                    };
                    self.waiting_payload = Some(m.clone());
                    self.bridge.send_to_async(m);
                }
            }
            if c.properties.contains(CharPropFlags::WRITE) {
                if ui.button("Write").clicked() {
                    let m = AsyncMsg::Payload {
                        payload: vec![],
                        char: c.clone(),
                        op: BLEOperation::WriteNoResponse,
                    };
                    self.waiting_payload = Some(m.clone());
                    self.bridge.send_to_async(m);
                }
            }
            if c.properties.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE) {
                if ui.button("Wr (w/o)").clicked() {
                    let m = AsyncMsg::Payload {
                        payload: vec![],
                        char: c.clone(),
                        op: BLEOperation::WriteWithResponse,
                    };
                    self.waiting_payload = Some(m.clone());
                    self.bridge.send_to_async(m);
                }
            }
            if c.properties.contains(CharPropFlags::NOTIFY) {
                if ui.button("Enale Notifs").clicked() {
                    let m = AsyncMsg::Payload {
                        payload: vec![],
                        char: c.clone(),
                        op: BLEOperation::EnableNotify,
                    };
                    self.waiting_payload = Some(m.clone());
                    self.bridge.send_to_async(m);
                }
            }
        });
    }

    pub fn draw_svc_table(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, svc_uuid: Uuid) {
        ui.collapsing(format!("Service: {svc_uuid:?}"), |ui| {
            let char_vec = self
                .svc_map
                .get(&svc_uuid)
                .expect("trying to get value from svc map")
                .clone();
            egui::Grid::new("some_unique_id")
                .striped(true)
                .show(ui, |ui| {
                    for c in char_vec.clone() {
                        ui.label(self.get_char_desc(c.uuid.clone()))
                            .on_hover_ui(|ui| {
                                ui.label(format!("{}", c.uuid));
                            });

                        ui.label(get_props_desc(c.properties));

                        self.draw_char_buttons(ui, c.clone());

                        if self.char_values.contains_key(&c.uuid) {
                            ui.label(format!(
                                "Read value: {:?}",
                                self.get_char_value_desc(c.uuid)
                            ))
                            .on_hover_ui(|ui| {
                                //
                            });
                        } else {
                            ui.label("n/a");
                        }

                        // end each row
                        ui.end_row();
                    }
                });

            // for c in char_vec {
            //     ui.label(format!("{} : {:?}", c.uuid, c.properties));
            // }
        });
    } // NOTE: end draw_svc_table

    pub fn get_char_desc(&mut self, u: Uuid) -> String {
        let default: String = format!("{u}");
        self.char_descs.get(&u).unwrap_or(&default).clone()
    }

    pub fn get_char_value_desc(&mut self, u: Uuid) -> String {
        match self.char_values.get(&u) {
            Some(u8_vec) => match String::from_utf8(u8_vec.clone()) {
                Ok(utf8_str) => utf8_str,
                Err(_bad) => {
                    format!("{u8_vec:?}")
                }
            },
            None => "n/a".into(),
        }
    }
} // NOTE: end: impl GuiApp

impl eframe::App for GuiApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.draw_top_panel(ctx);

        self.draw_central_panel(ctx);

        self.draw_bottom_panel(ctx);

        // repaint often, but sleep a bit...
        std::thread::sleep(Duration::from_millis(5));
        ctx.request_repaint();
    } // NOTE: end egui update
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
