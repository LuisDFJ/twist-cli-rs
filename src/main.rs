use std::time::Duration;
use std::thread;

use serialport;

fn main() {
    let mut port = serialport::new("/home/luisdfj/ttyUSB1", 115200)
        .timeout(Duration::from_millis(5))
        .open()
        .expect("Unable to open port");
    let mut buf : [u8;128] = [0;128];
    for _ in 0..10 {
        port.write("n\r\n".as_bytes())
            .expect("Could not write to port");
        thread::sleep(Duration::from_millis(500));
        if let Ok(n) = port.read( &mut buf ) {
            let res = String::from_utf8(Vec::from( &buf[..n] ) )
                .expect("Could not parse from utf8");
            println!("{n}-bytes read: {}", res);
        }
    }
}
