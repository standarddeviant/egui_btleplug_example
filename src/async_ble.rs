// use crate::transport::dfu_uuids::*;
// use crate::transport::DfuTransport;

use crate::async_msg::AsyncMsg;
use crate::async_msg::GenericResult::{ResultSuccess, ResultUnknown};

use btleplug::api::{Central, Manager, Peripheral as _, PeripheralProperties, ScanFilter};
use btleplug::platform::Adapter;
use btleplug::platform::Peripheral;
// use futures::stream::StreamExt;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::Duration;
// ::{Sender, Receiver};

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

    // TODO - organize in helper struct?

    let adapter = &adapter_list[0];
    let mut props_vec: Vec<(usize, PeripheralProperties)> = vec![];
    let mut scanned_periphs: Vec<Peripheral> = vec![];
    let mut connected_periph: Option<&Peripheral> = None;
    // let mut connected: &Peripheral;

    loop {
        match in_recv.recv().await {
            None => {}
            Some(AsyncMsg::ScanStart { filter, duration }) => {
                println!(
                    "async_ble: acting on ScanStart{{filter: {filter}, duration: {duration}}}"
                );
                let _ = adapter.start_scan(ScanFilter::default()).await?;
                tokio::time::sleep(Duration::from_secs_f32(duration)).await;

                match adapter.peripherals().await {
                    Ok(periphs) => {
                        props_vec.clear();
                        for (ix, periph) in periphs.clone().iter().enumerate() {
                            if let Ok(Some(props)) = periph.properties().await {
                                props_vec.push((ix, props));
                            }
                        }
                        scanned_periphs = periphs.clone();

                        // NOTE: hold on to local copy of periphs
                        // pvec = periphs.clone();
                        match out_send
                            .send(AsyncMsg::ScanResult {
                                result: ResultSuccess,
                                periphs: props_vec.clone(),
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

            Some(AsyncMsg::ConnectStart { index, periph }) => {
                match scanned_periphs[index].connect().await {
                    Ok(_good) => {
                        connected_periph = Some(&scanned_periphs[index]);
                        let res = out_send
                            .send(AsyncMsg::ConnectResult {
                                result: ResultSuccess,
                                index,
                                periph,
                            })
                            .await;
                        match res {
                            Ok(_good) => {}
                            Err(_bad) => {}
                        }

                        if let Some(p) = connected_periph {
                            match p.discover_services().await {
                                Ok(_good) => {
                                    let res = out_send
                                        .send(AsyncMsg::Characteristics {
                                            chars: p.characteristics(),
                                        })
                                        .await;
                                    match res {
                                        Ok(_good) => (),
                                        Err(_bad) => (),
                                    };
                                }
                                Err(_bad) => {}
                            }
                        }
                    }
                    Err(_bad) => {
                        eprintln!("Hmmm, didn't connect...: {_bad}");
                        match out_send
                            .send(AsyncMsg::ConnectResult {
                                result: ResultUnknown,
                                index: 0,
                                periph: PeripheralProperties::default(),
                            })
                            .await
                        {
                            Ok(_good) => (),
                            Err(_bad) => (),
                        };

                        //
                    }
                }
            }

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
