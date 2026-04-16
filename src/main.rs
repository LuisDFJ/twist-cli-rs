pub mod experiment;
pub mod serial_parser;
pub mod cmd_sender;
pub mod controller;
pub mod com_handler;
pub mod commands;
pub mod data;
pub mod view;

use std::io;
use crate::controller::Controller;
use crate::commands::Dir;
use crate::experiment::ExperimentParams;
use crate::serial_parser::SerialParser;

fn main() -> io::Result<()> {
    let parser = SerialParser::new("\r\n", "Ncm", "DEG");
    let params = ExperimentParams::new( "test_6", 0.25, Dir::CCW, 2, 10.0);
    let controller = Controller::new("/dev/ttyUSB0", 115200, parser, params)?;
    controller.run_tui()?;
    Ok(())
}
