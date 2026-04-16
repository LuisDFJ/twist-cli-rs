use std::io;
pub trait CmdSender<C> {
    fn send_cmd( self : &Self, cmd : C, millis : Option<u64> ) -> io::Result<()>;
}
