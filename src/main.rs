pub mod serial_parser;
pub mod controller;
pub mod com_handler;
pub mod commands;
pub mod data;

use std::time::Duration;
use std::thread;

use crate::controller::Controller;
use crate::com_handler::ComHandler;

fn main() {
    let controller = Controller::new("/home/luisdfj/ttyUSB1", 115200, "\r\n", "Deg", "Ncm");
    let com = ComHandler::new(&controller);
    thread::sleep(Duration::from_millis(100));
    drop(com)
}
