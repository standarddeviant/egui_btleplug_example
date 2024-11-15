// Note: use these types directly from rust<-> rust for now, but enable an external interface via
// JSON w/ validation

use serde::{Deserialize, Serialize};

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

// #[derive(Serialize, Deserialize)]
// struct MsgVersion {
//     major: i32,
//     minor: i32,
//     patch: i32,
// }
//
// #[derive(Serialize, Deserialize)]
// struct ConnectStart {
//     index: i32,
//     periph: String,
// }
//
// #[derive(Serialize, Deserialize)]
// struct ConnectResult {
//     result: GenericResult,
//     index: i32,
//     periph: String,
// }
//
// #[derive(Serialize, Deserialize)]
// struct Payload {
//     payload: Vec<u8>,
//     char: i32,
//     op: BLEOperation,
// }
//
// #[derive(Serialize, Deserialize)]
// struct DisconnectStart {
//     index: i32,
//     periph: String,
// }
// #[derive(Serialize, Deserialize)]
// struct DisconnectResult {

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AsyncMsg {
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
        periphs: Vec<String>,
    },
    ConnectStart {
        index: i32,
        periph: String,
    },
    ConnectResult {
        result: GenericResult,
        index: i32,
        periph: String,
    },
    Payload {
        payload: Vec<u8>,
        char: i32,
        op: BLEOperation,
    },
    DisconnectStart {
        index: i32,
        periph: String,
    },
    DisconnectResult {
        result: GenericResult,
        index: i32,
        periph: String,
    },
}
