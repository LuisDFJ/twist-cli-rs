use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender,Receiver};

use crate::controller::Controller;
use crate::data::Data;
use crate::commands::Cmd;

pub fn thread_control( controller : &Controller, tx : Sender<Cmd>, rx : Receiver<Data> ) -> thread::JoinHandle<()> {
    let flag = controller.flag.clone();

    let handler = thread::spawn( move || {
            if let Err(_) = tx.send(Cmd::Move(crate::commands::Dir::CCW)) {
            }
        
        loop {
            if let Err(_) = tx.send(Cmd::GetTorquePosition) {
                break;
            }
            println!("Asd:");
            loop {
                if let Ok(d) = rx.try_recv() {
                    println!("From Control: {:?}", d);
                } else {
                    break;
                }
            }

            if let Ok(f) = flag.try_read() {
                if !(*f) { break }
            }

            thread::sleep(Duration::from_millis(20));
        }
    });
    handler
}
