use std::time::Duration;
use std::thread;
use std::sync::{Arc,RwLock};

use serialport;

struct Controller {
    port : Box<dyn serialport::SerialPort>,
    flag  : Arc<RwLock<bool>>
}
struct ComHandler<'a> {
    controller : &'a Controller,
    t_listen : Option< thread::JoinHandle<()> >,
    t_work   : Option< thread::JoinHandle<()> >,
}

impl Controller {
    pub fn new( port : &str, baudrate : u32 ) -> Controller {
        let port = serialport::new( port, baudrate )
            .timeout(Duration::from_millis(1))
            .open()
            .expect("fail to open controller serial port");
        Controller { 
            port,
            flag: Arc::new( RwLock::new(false) )
        }
    }
    pub fn set_flag( self : &Self, flag : bool ) {
        let mut f = self.flag.write()
            .expect("fail to write to gaurd");
        *f = flag;
    }
}

impl <'a> ComHandler<'a> {
    pub fn new( controller : &'a Controller ) -> Self {
        controller.set_flag(true);
        let t_listen = thread_listen(controller);
        let t_work   = thread_work(controller);
        ComHandler{
            controller,
            t_listen: Some( t_listen ),
            t_work: Some( t_work )
        }
    }
}
impl <'a> Drop for ComHandler<'a> {
    fn drop(&mut self) {
        self.controller.set_flag(false);
        if let Some(h) = self.t_listen.take() {
            h.join().unwrap();
        }
        if let Some(h) = self.t_work.take() {
            h.join().unwrap();
        }
    }
}

fn thread_work( controller : &Controller ) -> thread::JoinHandle<()> {
    let mut port = controller.port.try_clone()
        .expect("fail to clone port");
    let flag = controller.flag.clone();
    let handler = thread::spawn( move || {
        loop {
            port.write("a\r\n".as_bytes())
                .expect("fail to write to serial port");

            if let Ok(f) = flag.try_read() {
                if !(*f) { break }
            }

            thread::sleep(Duration::from_millis(100));
        }
    });
    handler
}

fn thread_listen( controller : &Controller ) -> thread::JoinHandle<()> {
    let mut port = controller.port.try_clone()
        .expect("fail to clone port");
    let flag = controller.flag.clone();
    let handler = thread::spawn( move || {
        let mut buffer : Vec<u8> = vec![];
        loop {
            let mut buf = [0u8;64];
            if let Ok(t) = port.read(&mut buf) {
                if t > 0 {
                    buffer.append(&mut buf.to_vec());
                    if let Ok(buf) = String::from_utf8(buffer.clone()) {
                        println!("Asd: {}", buf);
                    }
                }
            }
            if let Ok(f) = flag.try_read() {
                if !(*f) { break }
            }

            thread::sleep(Duration::from_millis(1));
        }
    });
    handler
}

fn main() {
    let controller = Controller::new("/home/luisdfj/ttyUSB1", 115200);
    let com = ComHandler::new(&controller);
    thread::sleep(Duration::from_millis(1000));
    drop(com)
}
