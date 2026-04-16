use std::thread;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use std::io::{self,Error,ErrorKind};

use crate::commands::{Cmd,Mode,Speed,Unit};
use crate::data::Data;
use super::{Experiment,Control,State,XY};

impl Experiment {
    pub fn config( self : &mut Self, tx : &Sender<Cmd> ) -> io::Result<()> {
        tx.send_cmd(Cmd::Mode(Mode::Manual))?;
        tx.send_cmd(Cmd::Units(Unit::Deg))?;
        tx.send_cmd(Cmd::Speed(Speed::Prog))?;
        tx.send_cmd(Cmd::SetSpeed(self.speed))?;
        tx.send_cmd(Cmd::SetZero)?;
        tx.send_cmd(Cmd::Move(self.direction))?;
        self.state = State::Forward;
        Ok(())
    }
    pub fn pool( self : &mut Self, tx : &Sender<Cmd>, rx : &Receiver<Data> ) -> io::Result<bool> {
        tx.send_cmd(Cmd::GetTorquePosition)?;
        loop {
            if let Ok(d) = rx.try_recv() {
                match d {
                    Data::XYU(x,y,xu,yu) => {
                        self.data.push( XY{x,y} );
                        self.units.x.get_or_insert(xu);
                        self.units.y.get_or_insert(yu);
                    },
                    Data::Unknown(s) => println!("Unknown pattern: {}", s),
                    _ => {},
                }
            } else {
                break;
            }
        }
        if let Some(control_vec) = self.control() {
            for control_cmd in control_vec {
                match control_cmd {
                    Control::Command(cmd) => tx.send_cmd(cmd)?,
                    Control::Halt(time) => thread::sleep(Duration::from_millis(time as u64)),
                    Control::Break => return Ok(true),
                }
            }
        }
        Ok(false)
    }
    fn control( self : &mut Self ) -> Option<Vec<Control>> {
        let d = self.data.last()?;
        Some(State::step(self, d.x, d.y))
    }
}

trait CmdSender {
    fn send_cmd( self : &Self, cmd : Cmd ) -> io::Result<()>;
}

impl CmdSender for Sender<Cmd> {
    fn send_cmd( self : &Self, cmd : Cmd ) -> io::Result<()> {
        self.send(cmd)
            .map_err(|_| Error::new(ErrorKind::Other, "fail to send command"))?;
        Ok(())
    }
}
