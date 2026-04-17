use std::io;
use std::time::Duration;

use super::Controller;
use crate::{camera::CameraCapture, com_handler::ComHandler};

use crossterm::{event::{Event,KeyCode,KeyModifiers,self},terminal};

impl Controller {
    pub fn run( self : &Self ) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        let com = ComHandler::new(&self)?;
        let mut cam = CameraCapture::new(&self.cam_params, &self.params.dir )?;
        let t = Duration::from_millis(25);
        loop {
            if event::poll(t)? {
                if let Event::Key(key) =  event::read()? {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => break,
                        _ => {}
                    }
                }
            }
            let _ = cam.capture( self.params.speed as f64 );
            if let Ok(flag) = self.break_flag.try_read() {
                if *flag { break }
            }
        }
        terminal::disable_raw_mode()?;
        drop(com);
        Ok(())
    }
}

