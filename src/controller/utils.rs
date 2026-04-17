use super::Controller;

use std::io::{self,Error,ErrorKind};
use std::sync::Arc;

use crate::serial_parser::SerialParser;
use crate::commands::Cmd;
use crate::data::Data;

impl Controller {
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
        //if let Ok(s) = String::from_utf8(buffer.clone()){
            //if s.len() > 0 {
                //println!("{s}");
            //}
        //}
        Data::parse_buffer(parser, buffer)
    }
}
