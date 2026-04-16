use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self,Sender,Receiver};
use std::io;

use crate::controller::Controller;
use crate::commands::Cmd;

pub fn thread_send( controller : &Controller ) -> (thread::JoinHandle<io::Result<()>>, Sender<Cmd>) {

    let mut port = controller.port.try_clone()
        .expect("fail to clone port");
    let flag = controller.flag.clone();
    let (tx,rx) : (Sender<Cmd>, Receiver<Cmd>) = mpsc::channel();
    
    let handler = thread::spawn( move || {
        loop {
            if let Ok(cmd) = rx.try_recv() {
                Controller::send(&mut port, cmd)?;
            }

            if let Ok(f) = flag.try_read() {
                if !(*f) { break }
            }
            thread::sleep(Duration::from_millis(1));
        }
        Controller::send(&mut port, Cmd::Stop)?;
        Ok(())
    });
    ( handler, tx )
}
