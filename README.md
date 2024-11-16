# egui_btleplug_example

## Introduction
This is a simple-ish demo of how to use `egui` and `btleplug` together and make a 'bluetooth gui' in rust.

The *async* nature of `btleplug` presents minor challenges when integrating with `egui`.
I started with a simple program based on `tokio` 'bridging' guide here:
https://tokio.rs/tokio/topics/bridging

## Design

### Bridging
The chosen technique of 'bridging' for this demo is
> spawn async runtime in another thread and pass messages back and forth

This seemed like the most flexible approach since I want to grow the 
capabilities of this demo app to eventually include features like
- 'NUS terminal';
  - i.e. software that behaves like a connected UART shell, but over BLE. The acronym 'NUS' means *Nordic UART Service*.
- Arbitrary reads + writes on discovered characteristics
- Show Device Info

### Messaging Improvement
There is a single, bi-directional message enum that is sent across the bridge.

This means new features need new messages.
I'm not sure yet if this is good or bad...
It definitely requires more up-front thinking about
how to communicate between the GUI and the BLE logic, and/or adding to the message enum, `AsyncMsg`.
This might be a good thing to enforce some organization to the GUI.

It feels like `async_ble.rs` could be better organized to manage its state in accordance with `AsyncMsg`.

### GUI Improvement
The visual elements are literally one step away from `eframe_template` :-)
