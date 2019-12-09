use std::fmt;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Award {
    First = 5,  // master ball
    Second = 4, // rare candy
    Third = 3,  // pp max
    Fourth = 2, // pp up
    Fifth = 1,  // moomoo milk
    Losing = 0, // lose lottery...
}

impl fmt::Display for Award {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<11}",
            match self {
                Award::First => "Master Ball",
                Award::Second => "Rare Candy",
                Award::Third => "PP Max",
                Award::Fourth => "PP Up",
                Award::Fifth => "Moomoo Milk",
                Award::Losing => "Nothing",
            }
        )
    }
}
