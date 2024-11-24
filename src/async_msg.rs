// Note: use these types directly from rust<-> rust for now, but enable an external interface via
// JSON w/ validation

use btleplug::api::{Characteristic, PeripheralProperties};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

// #[derive(Serialize, Deserialize)]
// struct Person {
//     name: String,
//     age: u8,
//     phones: Vec<String>,
// }

// borrow generic result from gopro bluetooth defs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GenericResult {
    ResultUnknown = 0,
    ResultSuccess = 1,
    ResultIllFormed = 2,
    ResultNotSupported = 3,
    ResultArgumentOutOfBounds = 4,
    ResultArgumentInvalid = 5,
    ResultResourceNotAvailable = 6,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BLEOperation {
    Unknown = 0,
    Read = 1,
    WriteWithResponse = 2,
    WriteNoResponse = 3,
    EnableNotify = 4,
    Notify = 5,
    EnableIndicate = 6,
    Indicate = 7,
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum AsyncMsg {
    Error(String),
    MsgVersion {
        major: i32,
        minor: i32,
        patch: i32,
    },
    ScanStart {
        filter: String,
        duration: f32,
    },
    ScanResult {
        result: GenericResult,
        props_vec: Vec<(usize, PeripheralProperties)>,
        // periphs: Vec<String>,
    },
    ConnectStart {
        index: usize,
        props: PeripheralProperties,
    },
    ConnectResult {
        result: GenericResult,
        index: usize,
        props: PeripheralProperties,
    },
    Characteristics {
        chars: BTreeSet<Characteristic>,
    },
    DisconnectStart {
        index: usize,
        props: PeripheralProperties,
    },
    DisconnectResult {
        result: GenericResult,
        index: usize,
        props: PeripheralProperties,
    },
    Payload {
        payload: Vec<u8>,
        char: Characteristic,
        op: BLEOperation,
    },
    PayloadUuid {
        payload: Vec<u8>,
        uuid: Uuid,
        op: BLEOperation,
    },
}
