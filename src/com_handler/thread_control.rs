use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender,Receiver};
use std::io;

use crate::controller::Controller;
use crate::data::Data;
use crate::commands::Cmd;
use crate::experiment::Experiment;

pub fn thread_control( controller : &Controller, tx : Sender<Cmd>, rx : Receiver<Data> ) -> thread::JoinHandle<()> {
    let flag = controller.flag.clone();
    let break_flag = controller.break_flag.clone();
    let params = controller.params;
    let handler = thread::spawn( move || {
        let mut experiment = Experiment::new(params);
        control_loop(&mut experiment, tx, rx, flag, break_flag)
            .unwrap_or_else(|e| println!("control loop error: {e}"));
    });
    handler
}

fn control_loop( experiment : &mut Experiment, tx : Sender<Cmd>, rx : Receiver<Data>, flag : Arc<RwLock<bool>>, break_flag : Arc<RwLock<bool>> ) -> io::Result<()> {
    experiment.config(&tx)?;
    loop {
        if experiment.pool(&tx, &rx)? {
            match break_flag.try_write() {
                Ok(mut flag) => *flag = true,
                Err(e) => match e {
                    std::sync::TryLockError::Poisoned(_) => { break },
                    _ => {}
                },
            }
        }
        if let Ok(f) = flag.try_read() {
            if !(*f) { break }
        }
        thread::sleep(Duration::from_millis(20));
    }
    Ok(())
}

