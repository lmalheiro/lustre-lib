use crate::environment::Environment;
use crate::object::*;
use std::sync::mpsc::*;
use std::sync::Arc;
use std::thread::{Thread};

//struct DispatchingMessage(u64, RefObject);
//struct ResultMessage(u64, ResultRefObject);

struct DispatchingMessage(u64, RefObject);
struct ResultMessage(u64, ResultRefObject);

struct EvalWorkers {
    n_threads: i32,
    children: Vec<Thread>,
    to_workers: Vec<Sender<DispatchingMessage>>,
    from_workers: Vec<Receiver<ResultMessage>>,
}

impl <'a> EvalWorkers {
    pub fn new(n_threads: i32) -> Self {
        EvalWorkers {
            n_threads,
            children: Vec::new(),
            to_workers: Vec::new(),
            from_workers: Vec::new(),
        }
    }
    pub fn start(&mut self) {
        for i in 0..self.n_threads {
            let (to_worker, from_dispatcher): (
                Sender<DispatchingMessage>,
                Receiver<DispatchingMessage>,
            ) = channel();
            let (to_dispatcher, from_worker): (Sender<ResultMessage>, Receiver<ResultMessage>) =
                channel();
            self.to_workers.push(to_worker);
            self.from_workers.push(from_worker);
            std::thread::spawn(move || loop {
                let msg = from_dispatcher.recv().unwrap();

                to_dispatcher.send(ResultMessage(msg.0, Ok(msg.1))).unwrap();
            });
        }
    }
}
