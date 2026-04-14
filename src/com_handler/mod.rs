pub mod thread_control;
pub mod thread_send;
pub mod thread_listen;

use std::thread;

use crate::controller::Controller;
use self::thread_send::thread_send;
use self::thread_listen::thread_listen;
use self::thread_control::thread_control;

pub struct ComHandler<'a> {
    controller : &'a Controller,
    t_listen : Option< thread::JoinHandle<()> >,
    t_send   : Option< thread::JoinHandle<()> >,
    t_control: Option< thread::JoinHandle<()> >,
}

impl <'a> ComHandler<'a> {
    pub fn new( controller : &'a Controller ) -> Self {
        controller.set_flag(true);
        let (t_listen, rx) = thread_listen(controller);
        let (t_send, tx) = thread_send(controller);
        let t_control = thread_control(controller, tx, rx);

        ComHandler{
            controller,
            t_listen: Some( t_listen ),
            t_send: Some( t_send ),
            t_control: Some( t_control )
        }
    }
}
impl <'a> Drop for ComHandler<'a> {
    fn drop(&mut self) {
        self.controller.set_flag(false);
        if let Some(h) = self.t_listen.take() {
            h.join().unwrap();
        }
        if let Some(h) = self.t_send.take() {
            h.join().unwrap();
        }
        if let Some(h) = self.t_control.take() {
            h.join().unwrap();
        }
    }
}
