use std::sync::{Arc,RwLock};
use std::time::Duration;
use std::io::{self,Error,ErrorKind};

use crate::commands::Cmd;
use crate::data::Data;
use crate::experiment::ExperimentParams;
use crate::serial_parser::SerialParser;

pub struct Controller {
    pub port : Box<dyn serialport::SerialPort>,
    pub flag  : Arc<RwLock<bool>>,
    pub break_flag : Arc<RwLock<bool>>,
    pub parser : Arc<SerialParser>,
    pub params : ExperimentParams
}

use crate::com_handler::ComHandler;
impl Controller {
    pub fn run( self : &Self ) {
        let com = ComHandler::new(&self);
        loop {
            if let Ok(flag) = self.break_flag.try_read() {
                if *flag { break }
            }
        }
        drop(com);
    }
}

impl Controller {
    pub fn new( port : &str, baudrate : u32, parser : SerialParser, params : ExperimentParams ) -> io::Result<Self> {
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
            params
        } )
    }
    pub fn set_flag( self : &Self, flag : bool ) -> io::Result<()> {
        let mut f = self.flag.write()
            .map_err(|_| Error::new(ErrorKind::Other, "fail to write to flag guard"))?;
        *f = flag;
        Ok(())
    }
    pub fn send( port : &mut Box<dyn serialport::SerialPort>, cmd : Cmd ) -> io::Result<()>{
        port.write( cmd.parse().as_bytes() )
            .map_err(|_| Error::new(ErrorKind::Other, "fail to send message"))?;
        Ok(())
    }
    pub fn receive( port : &mut Box<dyn serialport::SerialPort>, parser : &Arc<SerialParser>, buffer : &mut Vec<u8> ) -> Option<Data> {
        let mut buf = [0u8;64];
        if let Ok(t) = port.read(&mut buf) {
            buffer.append(&mut buf[..t].to_vec());
        }
        if let Ok(s) = String::from_utf8(buffer.clone()){
            if s.len() > 0 {
                println!("{s}");
            }
        }
        Data::parse_buffer(parser, buffer)
    }
}
