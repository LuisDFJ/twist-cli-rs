use std::io;
use std::time::{Instant,Duration};

use super::Controller;
use crate::{camera::CameraCapture, com_handler::ComHandler};

use crossterm::{event::{Event,KeyCode,KeyModifiers,self},terminal};
use ratatui;

impl Controller {
    pub fn run_tui( self : &Self ) -> std::io::Result<()> {
        ratatui::run( | terminal | app(self,terminal) )
    }
}

use crate::view::Graph;
fn app( controller : &Controller, terminal : &mut ratatui::DefaultTerminal ) -> io::Result<()> {
    let com = ComHandler::new(&controller)?;
    let mut graph = Graph::new( controller.params.limit as f64, controller.params.direction );
    let mut cam = CameraCapture::new(&controller.cam_params, &controller.params.dir )?;
    let mut timer = Instant::now();
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
        let _ = cam.capture( controller.params.speed as f64 );
        graph.receive_cmd(&com.view_rx);
        if timer.elapsed() > t {
            timer = Instant::now();
            terminal.draw(|frame| frame.render_widget(&graph, frame.area()))?;
        }
        if let Ok(flag) = controller.break_flag.try_read() {
            if *flag { break }
        }
    }
    terminal::disable_raw_mode()?;
    drop(com);
    Ok(())
}
