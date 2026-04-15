use super::{Control,Experiment};
use crate::commands::Cmd;

pub enum State {
    Stop,
    Forward,
    Backward,
    Finish,
}

impl State {
    pub fn step( experiment : &mut Experiment, x : f32, y : f32 ) -> Vec<Control>{
        match experiment.state {
            State::Stop => State::step_stop(experiment),
            State::Forward => State::step_forward(experiment, x, y),
            Self::Backward => State::step_backward(experiment, x, y),
            Self::Finish => State::step_finish(experiment),
        }
    }
    fn step_stop( _ : &mut Experiment ) -> Vec<Control> {
        vec![]
    }
    fn step_forward( experiment : &mut Experiment, x : f32, _ : f32 ) -> Vec<Control> {
        let mut c = vec![];
        if x.abs() > experiment.limit {
            experiment.state = State::Backward;
            c.push( Control::Command(Cmd::Stop) );
            c.push( Control::Halt(1000) );
            c.push( Control::Command(Cmd::Move(experiment.direction.neg())) );
        }
        c
    }
    fn step_backward( experiment : &mut Experiment, x : f32, y : f32 ) -> Vec<Control> {
        let mut c = vec![];
        if x.abs() > 0.0 && y * x.signum() < 0.0 {
            experiment.ncycles += 1;
            c.push( Control::Command(Cmd::Stop) );
            c.push( Control::Halt(1000) );
            if experiment.ncycles < experiment.cycles {
                c.push(Control::Command(Cmd::Move(experiment.direction)));
                experiment.state = State::Forward;
            } else {
                experiment.state = State::Finish;
            }
        }
        c
    }
    fn step_finish( _ : &mut Experiment ) -> Vec<Control> {
        vec![Control::Break]
    }
}
