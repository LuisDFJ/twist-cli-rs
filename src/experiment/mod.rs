mod state;
mod utils;

use crate::commands::{Cmd,Dir};
use self::state::State;

pub enum Control {
    Command(Cmd),
    Halt(usize),
    Break,
}

#[derive(Debug, Clone)]
pub struct ExperimentParams {
    name : String,
    speed : f32,
    direction : Dir,
    cycles : u8,
    limit : f32,
}

impl ExperimentParams {
    pub fn new( name : &str, speed : f32, direction : Dir, cycles : u8, limit : f32 ) -> Self {
        let speed = speed.max(0.2).min(5.0);
        ExperimentParams { speed, direction, cycles, limit, name : name.to_string() }
    }
}

#[derive(Debug,Clone, Copy)]
pub struct XY { pub x : f32, pub y : f32 }
#[derive(Debug,Clone)]
pub struct UN { pub x : Option<String>, pub y : Option<String> }

pub struct Experiment {
    name : String,
    file : BufWriter<fs::File>,
    pub data : Vec<XY>,
    pub units : UN,
    speed : f32,
    direction : Dir,
    cycles : u8,
    ncycles : u8,
    limit : f32,
    state : State
}

impl Experiment {
    pub fn new( params : ExperimentParams ) -> io::Result<Self> {
        Ok( Experiment {
            file : create_results_file(&params.name)?,
            name : params.name,
            speed: params.speed,
            direction: params.direction,
            cycles: params.cycles,
            limit: params.limit,
            data: vec![],
            units: UN { x: None, y: None },
            ncycles : 0,
            state : State::Stop
        } )
    }
}

use std::fs;
use std::path::Path;
use std::io::{self,Error,ErrorKind,BufWriter,Write};
fn create_results_file( dir : &str ) -> io::Result<BufWriter<fs::File>> {
    let base_dir = Path::new("./results/").join(dir);
    if let Ok(_) = fs::create_dir_all(base_dir.clone()) {
        let f = fs::File::create_new(base_dir.join("res.csv"))
            .map_err( |_| Error::new(ErrorKind::Other, "fail to create new file") )?;
        Ok(BufWriter::new(f))
    } else {
        Err( Error::new(ErrorKind::Other, "fail to create dir") )
    }
}

impl Drop for Experiment {
    fn drop(&mut self) {
        println!("Saving Experiment {} in file {:?}", self.name, self.file);
        let _ = save_data(&mut self.file, &self.data, &self.units)
            .inspect_err(|e| println!("fail to save experiment: {e}"));
        println!("Experiment Saved!");
    }
}

fn save_data( file : &mut BufWriter<fs::File>, data : &Vec<XY>, units : &UN ) -> io::Result<()> {
    writeln!( file, "{},{}",
        units.x.clone().unwrap_or("x".to_string()).to_uppercase(),
        units.y.clone().unwrap_or("y".to_string()).to_uppercase()
    )?;
    for XY{x,y} in data {
        writeln!( file, "{x},{y}" )?;
    }
    file.flush()?;
    Ok(())
}
