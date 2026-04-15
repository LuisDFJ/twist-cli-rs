mod state;
mod utils;

use crate::commands::{Cmd,Dir};
use crate::data::Data;
use self::state::State;

pub enum Control {
    Command(Cmd),
    Halt(usize),
    Break,
}

#[derive(Debug, Clone, Copy)]
pub struct ExperimentParams {
    speed : f32,
    direction : Dir,
    cycles : u8,
    limit : f32,
}

impl ExperimentParams {
    pub fn new( speed : f32, direction : Dir, cycles : u8, limit : f32 ) -> Self {
        let speed = speed.max(0.5).min(5.0);
        ExperimentParams { speed, direction, cycles, limit }
    }
}

pub struct Experiment {
    data : Vec<Data>,
    speed : f32,
    direction : Dir,
    cycles : u8,
    ncycles : u8,
    limit : f32,
    state : State
}

impl Experiment {
    pub fn new( params : ExperimentParams ) -> Self {
        Experiment {
            speed: params.speed,
            direction: params.direction,
            cycles: params.cycles,
            limit: params.limit,
            data: vec![],
            ncycles : 0,
            state : State::Stop
        }
    }
}

