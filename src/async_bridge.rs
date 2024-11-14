use tokio::runtime::Builder;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Task {
    name: String,
    // info that describes the task
}

impl Task {
    pub fn new(name: String) -> Task {
        Self { name }
    }
}

async fn handle_task(task: Task, out_send: mpsc::Sender<Task>) {
    println!("Got task {}", task.name);

    let response = Task::new(format!("Response from {}", task.name));
    match out_send.send(response).await {
        Ok(_good) => (),
        Err(bad) => eprintln!("handle_task can't reply: {bad}"),
    }
}

// #[derive(Clone)]
pub struct TaskBridge {
    to_async: mpsc::Sender<Task>,
    from_async: mpsc::Receiver<Task>,
}

impl TaskBridge {
    pub fn new() -> TaskBridge {
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
                while let Some(task) = in_recv.recv().await {
                    tokio::spawn(handle_task(task, out_send.clone()));
                }

                // Once all senders have gone out of scope,
                // the `.recv()` call returns None and it will
                // exit from the while loop and shut down the
                // thread.
            });
        });

        TaskBridge {
            to_async: in_send,
            from_async: out_recv,
        }
    }

    pub fn send_to_async(&self, task: Task) {
        match self.to_async.blocking_send(task) {
            Ok(()) => {}
            Err(_) => panic!("The shared runtime has shut down."),
        }
    }

    pub fn try_recv_from_async(&mut self) -> Option<Task> {
        match self.from_async.try_recv() {
            Ok(good) => Some(good),
            Err(_bad) => None,
        }
    }

    // pub fn recv_from_async(&self, task: Task) ->
}
