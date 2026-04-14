use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self,Sender,Receiver};

use crate::controller::Controller;
use crate::data::Data;

pub fn thread_listen( controller : &Controller ) -> (thread::JoinHandle<()>, Receiver<Data>) {

    let mut port = controller.port.try_clone()
        .expect("fail to clone port");
    let flag = controller.flag.clone();
    let parser = controller.parser.clone();
    let (tx,rx) : (Sender<Data>, Receiver<Data>) = mpsc::channel();

    let handler = thread::spawn( move || {
        let mut buffer : Vec<u8> = vec![];
        loop {
            if let Some(d) = Controller::receive(&mut port, &parser, &mut buffer) {
                if let Ok(_) = tx.send(d) {}
                else { break }
            }

            if let Ok(f) = flag.try_read() {
                if !(*f) { break }
            }
            //thread::sleep(Duration::from_millis(10));
        }
    });
    (handler, rx)
}
