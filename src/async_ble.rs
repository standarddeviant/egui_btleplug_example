// use crate::transport::dfu_uuids::*;
// use crate::transport::DfuTransport;

use crate::async_msg::AsyncMsg;
use crate::async_msg::GenericResult::ResultSuccess;

use btleplug::api::{Central, Manager, ScanFilter};
use btleplug::platform::Adapter;
use btleplug::platform::Peripheral;
// use futures::stream::StreamExt;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::Duration;
// ::{Sender, Receiver};

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

    let adapter = &adapter_list[0];
    let mut pvec: Vec<Peripheral>;

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
                        // NOTE: hold on to local copy of periphs
                        pvec = periphs.clone();
                        match out_send
                            .send(AsyncMsg::ScanResult {
                                result: ResultSuccess,
                                periphs,
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

            Some(unhandled) => {
                println!("async_ble got (UNHANDLED): {unhandled:?}");
            } // NOTE: end: Some(unhandled)
        }
    }
}

// EOF
