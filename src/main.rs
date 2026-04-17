pub mod experiment;
pub mod serial_parser;
pub mod cmd_sender;
pub mod controller;
pub mod com_handler;
pub mod commands;
pub mod data;
pub mod view;
pub mod camera;

use std::io;
use crate::controller::Controller;
use crate::commands::Dir;
use crate::experiment::ExperimentParams;
use crate::serial_parser::SerialParser;
use crate::camera::CameraParams;


fn main() -> io::Result<()> {
    let cam_params = CameraParams::new("C270 HD WEBCAM", 1000);
    let parser = SerialParser::new("\r\n", "Ncm", "DEG");
    let params = ExperimentParams::new( "c", 1.0, Dir::CW, 2, 10.0);
    let controller = Controller::new("/home/luisdfj/ttyUSB1", 115200, parser, params, cam_params)?;
    controller.run_tui()?;
    println!("Experiment Finished");
    Ok(())
}
