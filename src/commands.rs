#[derive(Debug)]
pub enum Dir { CW, CCW }

#[derive(Debug)]
pub enum Mode { Manual, Limit, Cycle }

#[derive(Debug)]
pub enum Unit { Rev, Deg }

#[derive(Debug)]
pub enum Speed { Prog, Max, Min }

#[derive(Debug)]
pub enum Cmd {
    GetSpeed, GetPosition, GetTorquePosition,
    Move(Dir), Units(Unit), Mode(Mode), Speed(Speed),
    SetSpeed(f32), SetZero,
    Stop
}

impl Cmd {
    pub fn parse( self : &Self ) -> String {
        let aux : String;
        match self {
            Cmd::GetSpeed => "a",
            Cmd::GetPosition => "x",
            Cmd::GetTorquePosition => "n",
            Cmd::Move(d) => match d {
                Dir::CW => "u",
                Dir::CCW => "d",
            },
            Cmd::Units(u) => match u {
                Unit::Rev => "b",
                Unit::Deg => "i",
            },
            Cmd::Mode(m) => match m {
                Mode::Manual => "m",
                _ => "m",
            },
            Cmd::Speed(s) => match s {
                Speed::Prog => "o",
                Speed::Max => "j",
                Speed::Min => "k",
            },
            Cmd::SetSpeed(s) => {
                aux = format!("e{:05.2}", s);
                &aux
            },
            Cmd::SetZero => "z",
            Cmd::Stop => "s",
        }.to_string() + "\r\n"
    }
}
