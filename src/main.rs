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
//use crate::controller::Controller;
//use crate::commands::Dir;
//use crate::experiment::ExperimentParams;
//use crate::serial_parser::SerialParser;
use crate::camera::CameraCapture;


fn main() -> io::Result<()> {
    let mut cam = CameraCapture::new("UVC Camera", 500, ".")?;
    loop {
        cam.capture()?;
        if cam.count > 10 { break }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    //let parser = SerialParser::new("\r\n", "Ncm", "DEG");
    //let params = ExperimentParams::new( "b", 1.0, Dir::CW, 2, 10.0);
    //let controller = Controller::new("/home/luisdfj/ttyUSB1", 115200, parser, params)?;
    //controller.run_tui()?;
    Ok(())
}
