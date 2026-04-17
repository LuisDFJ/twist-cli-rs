pub mod run;
pub mod run_tui;
pub mod utils;

use std::sync::{Arc,RwLock};

use crate::experiment::ExperimentParams;
use crate::serial_parser::SerialParser;
use crate::camera::CameraParams;

pub struct Controller {
    pub port : Box<dyn serialport::SerialPort>,
    pub flag  : Arc<RwLock<bool>>,
    pub break_flag : Arc<RwLock<bool>>,
    pub parser : Arc<SerialParser>,
    pub params : ExperimentParams,
    pub cam_params : CameraParams,
}

use std::io::{self,Error,ErrorKind};
use std::time::Duration;
impl Controller {
    pub fn new( port : &str, baudrate : u32, parser : SerialParser, params : ExperimentParams, cam_params : CameraParams ) -> io::Result<Self> {
        let port = serialport::new( port, baudrate )
            .timeout(Duration::from_millis(1))
            .open()
            .map_err(|_| Error::new(ErrorKind::Other, "fail to open controller serial port"))?;
        port.clear(serialport::ClearBuffer::All)
            .map_err(|_| Error::new(ErrorKind::Other, "fail to clear serial port"))?;
        Ok( Controller { 
            port,
            flag: Arc::new( RwLock::new(false) ),
            break_flag: Arc::new( RwLock::new(false) ),
            parser: Arc::new( parser ),
            params,
            cam_params
        } )
    }
}

