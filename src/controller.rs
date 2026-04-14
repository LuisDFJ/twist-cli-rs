use std::sync::{Arc,RwLock};
use std::time::Duration;

use crate::commands::Cmd;
use crate::data::Data;
use crate::serial_parser::SerialParser;

pub struct Controller {
    pub port : Box<dyn serialport::SerialPort>,
    pub flag  : Arc<RwLock<bool>>,
    pub parser : Arc<SerialParser>
}

impl Controller {
    pub fn new( port : &str, baudrate : u32, sep : &str, xun : &str, yun : &str ) -> Self {
        let port = serialport::new( port, baudrate )
            .timeout(Duration::from_millis(1))
            .open()
            .expect("fail to open controller serial port");
        port.clear(serialport::ClearBuffer::All)
            .expect("fail to clear serial port");
        Controller { 
            port,
            flag: Arc::new( RwLock::new(false) ),
            parser: Arc::new( SerialParser::new( sep, xun, yun ) )
        }
    }
    pub fn set_flag( self : &Self, flag : bool ) {
        let mut f = self.flag.write()
            .expect("fail to write to guard");
        *f = flag;
    }
    pub fn send( port : &mut Box<dyn serialport::SerialPort>, cmd : Cmd ) {
        port.write( cmd.parse().as_bytes() )
            .expect("fail to send message");
    }
    pub fn receive( port : &mut Box<dyn serialport::SerialPort>, parser : &Arc<SerialParser>, buffer : &mut Vec<u8> ) -> Option<Data> {
        let mut buf = [0u8;64];
        if let Ok(t) = port.read(&mut buf) {
            if t > 0 {
                buffer.append(&mut buf[..t].to_vec());
                if let Ok(s) = String::from_utf8(buffer.clone()){
                    println!("{s}");
                }
                return Data::parse_buffer(parser, buffer);
            }
        }
        None
    }
}
