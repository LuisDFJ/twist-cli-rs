use std::{io, time::Duration};

use super::Controller;

use crossterm::{event::{Event,KeyCode,KeyModifiers,self},terminal};
use ratatui;
use crate::com_handler::ComHandler;
impl Controller {
    pub fn run( self : &Self ) -> std::io::Result<()> {
        terminal::enable_raw_mode()?;
        let com = ComHandler::new(&self)?;
        loop {
            if event::poll(Duration::from_millis(10))? {
                if let Event::Key(key) =  event::read()? {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => break,
                        _ => {}
                    }
                }
            }
            if let Ok(flag) = self.break_flag.try_read() {
                if *flag { break }
            }
        }
        terminal::disable_raw_mode()?;
        drop(com);
        Ok(())
    }
    pub fn run_tui( self : &Self ) -> std::io::Result<()> {
        ratatui::run( | terminal | app(self,terminal) )
    }
}

use crate::view::Graph;
fn app( controller : &Controller, terminal : &mut ratatui::DefaultTerminal ) -> io::Result<()> {
    let com = ComHandler::new(&controller)?;
    let mut graph = Graph::new();
    loop {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) =  event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => break,
                    _ => {}
                }
            }
        }
            graph.receive_cmd(&com.view_rx);
            terminal.draw(|frame| frame.render_widget(&graph, frame.area()))?;
        if let Ok(flag) = controller.break_flag.try_read() {
            if *flag { break }
        }
    }
    terminal::disable_raw_mode()?;
    drop(com);
    Ok(())
}
