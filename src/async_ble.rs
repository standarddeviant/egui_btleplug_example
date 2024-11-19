// use crate::transport::dfu_uuids::*;
// use crate::transport::DfuTransport;

use crate::async_msg::GenericResult::{ResultSuccess, ResultUnknown};
use crate::async_msg::{AsyncMsg, BLEOperation};

use btleplug::api::{Central, Manager, Peripheral as _, PeripheralProperties, ScanFilter};
use btleplug::platform::Adapter;
use btleplug::platform::Peripheral;
// use futures::stream::StreamExt;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::Duration;
// ::{Sender, Receiver};

struct AsyncBLE {
    adapter: Adapter,
    scanned_props: Vec<(usize, PeripheralProperties)>,
    scanned_periphs: Vec<Peripheral>,
    connected_index: Option<usize>,
}

// // this is a pure function; output is a strict function of input
// fn periph_desc_string(props: &PeripheralProperties) -> String {
//     let mut dlist: Vec<String> = vec![]; // desc list
//     // name first
//     if let Some(name) = &props.local_name {
//         dlist.push(format!("name={}", name));
//     }
//
//     // rssi next
//     if let Some(rssi_val) = &props.rssi {
//         dlist.push(format!("rssi={}", rssi_val));
//     }
//
//     // addr next
//     dlist.push(format!("addr={}", props.address));
//
//     // return a joined version as the output
//     dlist.join(" : ")
// }

use std::error::Error;

pub async fn unwrap_adapter() -> Adapter {
    let manager = btleplug::platform::Manager::new().await.unwrap();
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().next().unwrap()
}

pub async fn scan_for_peripherals(
    adapter: &Adapter,
    // _filter_string: &str,
) -> Result<Vec<Peripheral>, Box<dyn Error>> {
    let _ = adapter.start_scan(ScanFilter::default()).await?;
    // central.start_scan(ScanFilter::default()).await?

    // TODO: clean this up...
    return Ok(adapter.peripherals().await.unwrap());
}

// ble_transport(out_send, in_recv).await;
pub async fn ble_transport_task(
    out_send: Sender<AsyncMsg>,
    mut in_recv: Receiver<AsyncMsg>,
) -> Result<(), Box<dyn Error>> {
    let manager = btleplug::platform::Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
        let _ = out_send
            .send(AsyncMsg::Error("No Bluetooth adapters found".into()))
            .await;

        // TODO: actually return an error here...
        return Ok(());
    }

    let mut ble = AsyncBLE {
        adapter: adapter_list[0].clone(),
        scanned_props: vec![],
        scanned_periphs: vec![],
        connected_index: None,
    };

    // TODO - organize in helper struct?

    loop {
        match in_recv.recv().await {
            None => {}
            Some(AsyncMsg::ScanStart { filter, duration }) => {
                println!(
                    "async_ble: acting on ScanStart{{filter: {filter}, duration: {duration}}}"
                );
                match ble.adapter.start_scan(ScanFilter::default()).await {
                    Ok(_good) => {}
                    Err(_bad) => {
                        eprintln!("hmmm... {_bad}");
                    }
                }
                tokio::time::sleep(Duration::from_secs_f32(duration)).await;
                match ble.adapter.stop_scan().await {
                    Ok(_good) => {}
                    Err(_bad) => {
                        eprintln!("hmmm... {_bad}")
                    }
                }

                match ble.adapter.peripherals().await {
                    Ok(periphs) => {
                        ble.scanned_periphs.clear();
                        ble.scanned_props.clear();
                        for (ix, periph) in periphs.clone().iter().enumerate() {
                            if let Ok(Some(props)) = periph.properties().await {
                                ble.scanned_props.push((ix, props));
                            }
                        }
                        ble.scanned_periphs = periphs.clone();

                        // NOTE: hold on to local copy of periphs
                        // pvec = periphs.clone();
                        match out_send
                            .send(AsyncMsg::ScanResult {
                                result: ResultSuccess,
                                props_vec: ble.scanned_props.clone(),
                            })
                            .await
                        {
                            Ok(_good) => (),
                            Err(_bad) => (),
                        }
                    }
                    Err(_bad) => {}
                };
            } // NOTE: end: got AsyncMsg::ScanStart { filter, duration }

            Some(AsyncMsg::ConnectStart { index, props }) => {
                println!("async_ble: acting on ConnectStart{{ {index}, {props:?} }}");
                let mut break_outer: bool = false;
                for _try in 0..5 {
                    if let Ok(_good) = ble.scanned_periphs[index].connect().await {
                        ble.connected_index = Some(index);
                        match out_send
                            .send(AsyncMsg::ConnectResult {
                                result: ResultSuccess,
                                index,
                                props: props.clone(),
                            })
                            .await
                        {
                            Ok(_good) => {}
                            Err(_bad) => {}
                        }

                        for _try2 in 0..5 {
                            if let Ok(_good) = ble.scanned_periphs[index].discover_services().await
                            {
                                match out_send
                                    .send(AsyncMsg::Characteristics {
                                        chars: ble.scanned_periphs[index].characteristics(),
                                    })
                                    .await
                                {
                                    Ok(_good) => (),
                                    Err(_bad) => (),
                                };
                                break_outer = true;
                                break;
                            }
                        }
                    }
                    if break_outer {
                        break;
                    }
                }
            } // NOTE: end: Some(AsyncMsg::ConnectStart { index, props }) => {...}

            Some(AsyncMsg::Payload { payload, char, op }) => match op {
                BLEOperation::Read => {
                    println!("async_ble: acting on Payload{{ payload: {payload:?}, char: {char:?}, op: {op:?} }}");
                    if let Some(ci) = ble.connected_index {
                        println!("async_ble: await on read...");
                        match ble.scanned_periphs[ci].read(&char).await {
                            Ok(rdbuf) => {
                                let resp = AsyncMsg::Payload {
                                    payload: rdbuf,
                                    char: char.clone(),
                                    op: crate::async_msg::BLEOperation::EnableIndicate,
                                };
                                match out_send.send(resp).await {
                                    Ok(_good) => {}
                                    Err(_bad) => {
                                        eprintln!("hmmm... {_bad}");
                                    }
                                };
                            }
                            Err(_bad) => {
                                eprintln!("hmmm... {_bad}");
                            }
                        }
                    }
                }
                BLEOperation::EnableNotify => {
                    println!("async_ble: acting on Payload{{ payload: {payload:?}, char: {char:?}, op: {op:?} }}");
                    if let Some(ci) = ble.connected_index {
                        println!("(TODO) Subscribing to characteristic {:?}", char.uuid);

                        ble.scanned_periphs[ci].subscribe(&char).await?;
                        // ble.notif_streams = ble.scanned_periphs[ci].notifications();
                        // ble.start_notif_watcher();

                        // let mut notification_stream = peripheral.notifications().await?.take(4);
                        // // Print the first 4 notifications received.
                        // let mut notification_stream =
                        //     self.scanned_periphs[ci].subscribe(&char).await?;

                        // peripheral.notifications().await?.take(4);
                        // Process while the BLE connection is not broken or stopped.
                        // while let Some(data) = notification_stream.next().await {
                        //     println!(
                        //         "Received data from {:?} [{:?}]: {:?}",
                        //         local_name, data.uuid, data.value
                        //     );
                        // }
                    }
                }
                _ => {
                    println!("ble_async: got (UNHANDLED) Payload{{ payload: {payload:?}, op: {op:?}, char: {char} }}");
                }
            },

            Some(AsyncMsg::DisconnectStart { index, props }) => {
                println!("async_ble acting on AsyncMsg::DisconnectStart{{ {index}, {props:?} }}");

                match ble.connected_index {
                    None => {
                        // TODO: handle discon req while async_ble is unconnected
                    }
                    Some(ci) => {
                        match ble.scanned_periphs[ci].disconnect().await {
                            Ok(_good) => {
                                let res = out_send
                                    .send(AsyncMsg::DisconnectResult {
                                        result: ResultSuccess,
                                        index: ci,
                                        props: ble.scanned_props[ci].1.clone(),
                                    })
                                    .await;
                                match res {
                                    Ok(_good) => (),
                                    Err(_bad) => (),
                                }

                                // "async_ble got AsyncMsg::DisconnectStart{{ {index}, {props:?} }}"
                                // );
                            }
                            Err(_bad) => (),
                        }
                    }
                }
                // ble.connected_periph
            } // NOTE: end: Some(unhandled)
            // unwrap().disconnect().await;
            // if let Some(p) = ble.connected_periph {
            // match ble.connected_periph.as_mut().unwrap().disconnect().await {
            //     // unwrap().disconnect().await {
            //     Ok(_good) => {}
            //     Err(_bad) => (),
            // }
            // }

            // Some(AsyncMsg::ReadDeviceInfo { filter, duration }) => {
            //     //
            // }

            // Some(AsyncMsg::Error)
            // filter, duration }) => {}
            Some(unhandled) => {
                println!("async_ble got (UNHANDLED): {unhandled:?}");
            } // NOTE: end: Some(unhandled)
        }
    }
}

// EOF
