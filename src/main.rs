pub mod experiment;
pub mod serial_parser;
pub mod controller;
pub mod com_handler;
pub mod commands;
pub mod data;

use std::io;
use crate::controller::Controller;
use crate::commands::Dir;
use crate::experiment::ExperimentParams;
use crate::serial_parser::SerialParser;

fn main() -> io::Result<()> {
    let parser = SerialParser::new("\r\n", "DEG", "ncm");
    let params = ExperimentParams::new( "P02", 1.0, Dir::CW, 2, 10.0);
    let controller = Controller::new("/home/luisdfj/ttyUSB1", 115200, parser, params)?;
    controller.run()?;
    Ok(())
}
