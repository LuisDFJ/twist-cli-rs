use std::sync::mpsc::Receiver;
use super::{Graph,ViewCmd};

impl Graph {
    pub fn receive_cmd( self : &mut Self, rx : &Receiver<ViewCmd> ) -> Option<()> {
        match rx.try_recv().ok()? {
            ViewCmd::Units(units) => {
                self.units = units;
            },
            ViewCmd::Entry(entry) => {
                let x : f64 = entry.x as f64;
                let y : f64 = entry.y as f64;
                self.data.push((x,y));
                self.x_limits.0 = x.min(self.x_limits.0);
                self.x_limits.1 = x.max(self.x_limits.1);
                self.y_limits.0 = y.min(self.y_limits.0);
                self.y_limits.1 = y.max(self.y_limits.1);
            },
        }
        Some(())
    }
}
