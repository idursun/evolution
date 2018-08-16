use rand::distributions::Standard;
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Team {
    Blue,
    Red,
    Yellow,
    Cyan,
}

impl Distribution<Team> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Team {
        match rng.gen_range(0, 4) {
            0 => Team::Blue,
            1 => Team::Red,
            2 => Team::Yellow,
            3 => Team::Cyan,
            _ => unreachable!(),
        }
    }
}
