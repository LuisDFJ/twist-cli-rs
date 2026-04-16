use std::sync::{Arc, RwLock, mpsc};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender,Receiver};
use std::io::{self, Error, ErrorKind};

use crate::controller::Controller;
use crate::view::ViewCmd;
use crate::data::Data;
use crate::commands::Cmd;
use crate::experiment::{Experiment,ExperimentParams};

pub fn thread_control( controller : &Controller, tx : Sender<Cmd>, rx : Receiver<Data> ) -> (thread::JoinHandle<()>, Receiver<ViewCmd>) {
    let (view_tx, view_rx) : (Sender<ViewCmd>,Receiver<ViewCmd>) = mpsc::channel();

    let flag = controller.flag.clone();
    let break_flag = controller.break_flag.clone();
    let params = controller.params.clone();
    let handler = thread::spawn( move || {
        control_loop(params, tx, rx, view_tx, flag, break_flag.clone())
            .unwrap_or_else(|e| println!("control loop error: {e}"));
        if let Ok(mut f) = break_flag.write() {
            *f = true;
        }
    });
    ( handler, view_rx )
}

fn control_loop( params : ExperimentParams, tx : Sender<Cmd>, rx : Receiver<Data>, view_tx : Sender<ViewCmd>, flag : Arc<RwLock<bool>>, break_flag : Arc<RwLock<bool>> ) -> io::Result<()> {
    let mut timer = std::time::Instant::now();
    let mut experiment = Experiment::new(params)?;
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
        if timer.elapsed() > Duration::from_millis(500) {
            timer = std::time::Instant::now();
            view_tx.send_cmd(ViewCmd::Units(experiment.units.clone()), None)?;
            if let Some(entry) = experiment.data.last() {
                view_tx.send_cmd(ViewCmd::Entry(*entry), None)?;
            }
        }
    }
    Ok(())
}

use crate::cmd_sender::CmdSender;
impl CmdSender<ViewCmd> for Sender<ViewCmd> {
    fn send_cmd( self : &Self, cmd : ViewCmd, millis : Option<u64> ) -> io::Result<()> {
        self.send( cmd )
            .map_err(|_| Error::new(ErrorKind::Other, "fail to send view command"))?;
        thread::sleep(Duration::from_millis(millis.unwrap_or_default()));
        Ok(())
    }
}
