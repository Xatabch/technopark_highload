use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub enum Message {
    NewTask(Task),
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub type Task = Box<FnBox + Send + 'static>;

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
    Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewTask(task) => {
                        println!("Worker {} got a job; executing.", id);
                        task.call_box();
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}