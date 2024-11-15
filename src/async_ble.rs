// use crate::transport::dfu_uuids::*;
// use crate::transport::DfuTransport;

use crate::async_msg::AsyncMsg;
use crate::async_msg::GenericResult::ResultSuccess;

use btleplug::api::{Central, Manager as _, ScanFilter};
use btleplug::platform::Adapter;
use btleplug::platform::Peripheral;
// use futures::stream::StreamExt;

use tokio::sync::mpsc::{Receiver, Sender};
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
pub async fn ble_transport_task(out_send: Sender<AsyncMsg>, mut in_recv: Receiver<AsyncMsg>) {
    loop {
        match in_recv.recv().await {
            None => {}
            Some(AsyncMsg::ScanStart { filter, duration }) => {
                // ) { filter, duration }) => {
                println!("async_ble got ScanStart{{filter: {filter}, duration: {duration}}}");
                let response = AsyncMsg::ScanResult {
                    result: ResultSuccess,
                    periphs: vec![],
                };
                match out_send.send(response).await {
                    Ok(_good) => (),
                    Err(bad) => eprintln!("ble_transport_task can't reply: {bad}"),
                }
            }
            Some(unhandled) => {
                println!("async_ble got (UNHANDLED): {unhandled:?}");
            }
        }
    }
}

// EOF
