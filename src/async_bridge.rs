use tokio::runtime::Builder;
use tokio::sync::mpsc;

use crate::async_msg::AsyncMsg;

use crate::async_ble::ble_transport_task;

// #[derive(Debug)]
// pub struct AsyncMsg {
//     name: String,
//     // info that describes the task
// }

// impl AsyncMsg {
//     pub fn new(name: String) -> AsyncMsg {
//         Self { name }
//     }
// }

// async fn handle_task(msg: AsyncMsg, out_send: mpsc::Sender<AsyncMsg>) {
//     println!("Got msg {:?}", msg);
//
//     // let response = AsyncMsg::new(format!("Response from {}", task.name));
//     // match out_send.send(response).await {
//     //     Ok(_good) => (),
//     //     Err(bad) => eprintln!("handle_task can't reply: {bad}"),
//     // }
// }

// #[derive(Clone)]
pub struct AsyncBridge {
    to_async: mpsc::Sender<AsyncMsg>,
    from_async: mpsc::Receiver<AsyncMsg>,
}

impl AsyncBridge {
    pub fn new() -> AsyncBridge {
        // Set up a channel for communicating.
        let (in_send, mut in_recv) = mpsc::channel(16);
        let (out_send, out_recv) = mpsc::channel(16);

        // Build the runtime for the new thread.
        //
        // The runtime is created before spawning the thread
        // to more cleanly forward errors if the `unwrap()`
        // panics.
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        std::thread::spawn(move || {
            rt.block_on(async move {
                ble_transport_task(out_send, in_recv).await;
            });
        });

        AsyncBridge {
            to_async: in_send,
            from_async: out_recv,
        }
    }

    pub fn send_to_async(&self, msg: AsyncMsg) {
        match self.to_async.blocking_send(msg) {
            Ok(()) => {}
            Err(_) => panic!("The shared runtime has shut down."),
        }
    }

    pub fn try_recv_from_async(&mut self) -> Option<AsyncMsg> {
        match self.from_async.try_recv() {
            Ok(good) => Some(good),
            Err(_bad) => None,
        }
    }

    // pub fn recv_from_async(&self, task: Task) ->
}
