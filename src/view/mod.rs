pub mod render;
pub mod utils;

use crate::experiment::{XY,UN};

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
    pub fn new() -> Self {
        Graph {
            data: vec![],
            units: UN { x: None, y: None },
            x_limits: (0.0,1.0),
            y_limits: (0.0,1.0)
        }
    }
}

