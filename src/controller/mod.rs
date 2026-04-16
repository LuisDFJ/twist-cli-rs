pub mod run;
pub mod utils;

use std::sync::{Arc,RwLock};

use crate::experiment::ExperimentParams;
use crate::serial_parser::SerialParser;

pub struct Controller {
    pub port : Box<dyn serialport::SerialPort>,
    pub flag  : Arc<RwLock<bool>>,
    pub break_flag : Arc<RwLock<bool>>,
    pub parser : Arc<SerialParser>,
    pub params : ExperimentParams
}


