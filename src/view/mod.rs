pub mod render;
pub mod utils;

use crate::experiment::{XY,UN};
use crate::commands::Dir;

pub enum ViewCmd {
    Entry(XY),
    Units(UN),
}

pub struct Graph {
    data : Vec<(f64,f64)>,
    units : UN,
    x_limits : (f64,f64),
    y_limits : (f64,f64),
}

impl Graph {
    pub fn new( limit : f64, direction : Dir ) -> Self {
        let x_limits = match direction {
            Dir::CW => (0.0, limit),
            Dir::CCW => (-limit, 0.0),
        };
        Graph {
            data: vec![(0.0,0.0)],
            units: UN { x: None, y: None },
            x_limits,
            y_limits: (0.0,1.0)
        }
    }
}

