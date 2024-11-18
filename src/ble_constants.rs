use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use uuid::Uuid;

pub fn get_default_char_descs() -> HashMap<Uuid, String, RandomState> {
    let kv_pairs = vec![
        (
            Uuid::from_u128(0x0000180A_0000_1000_8000_00805F9B34FB),
            "Device Info Service",
        ),
        (
            Uuid::from_u128(0x00002A24_0000_1000_8000_00805F9B34FB),
            "Model",
        ),
        (
            Uuid::from_u128(0x00002A25_0000_1000_8000_00805F9B34FB),
            "Serial Number",
        ),
        (
            Uuid::from_u128(0x00002A26_0000_1000_8000_00805F9B34FB),
            "Firmware Version",
        ),
        (
            Uuid::from_u128(0x00002A27_0000_1000_8000_00805F9B34FB),
            "Hardware Version",
        ),
        (
            Uuid::from_u128(0x00002A28_0000_1000_8000_00805F9B34FB),
            "Software Version",
        ),
        (
            Uuid::from_u128(0x00002A29_0000_1000_8000_00805F9B34FB),
            "Manufacturer",
        ),
    ];
    HashMap::<Uuid, String, RandomState>::from_iter(
        kv_pairs.iter().map(|kv| (kv.0, kv.1.to_string())),
    )
}
