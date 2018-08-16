extern crate rand;
use gene::{CellInstruction, Gene};
use rand::Rng;
use team::Team;

const DEFAULT_ANT_ENERGY: i32 = 50;
const DEFAULT_FOOD_ENERGY: i32 = 150;

#[derive(Debug)]
pub enum Action {
    Move,
    Eat,
    Attack,
}

#[derive(Debug, Clone)]
pub struct Ant {
    pub sensor: bool,
    pub team: Team,
    pub age: usize,
    pub energy: i32,
    pub direction: Direction,
    pub gene: Gene,
}

impl Ant {
    pub fn new() -> Ant {
        Ant {
            sensor: false,
            team: rand::random(),
            age: 0,
            energy: DEFAULT_ANT_ENERGY,
            direction: Direction::North,
            gene: Gene::random(),
        }
    }

    pub fn increase_energy(&mut self) {
        self.energy += DEFAULT_FOOD_ENERGY;
    }

    pub fn consume_energy(&mut self, amount: i32) {
        self.energy -= amount;
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, self.gene.code.len());
        self.gene.code[index] = rand::random();
    }

    pub fn execute(&mut self) -> Option<Action> {
        self.age += 1;
        match self.gene.cycle() {
            CellInstruction::JmpNe(target) if !self.sensor => {
                self.gene.instruction_pointer = target;
                None
            }
            CellInstruction::Jmp(target) if self.sensor => {
                self.gene.instruction_pointer = target;
                None
            }
            CellInstruction::TurnLeft => {
                self.direction = self.direction.turn_left();
                None
            }
            CellInstruction::TurnRight => {
                self.direction = self.direction.turn_right();
                None
            }
            CellInstruction::Attack => Some(Action::Attack),
            CellInstruction::Move => Some(Action::Move),
            CellInstruction::Eat => Some(Action::Eat),
            _ => None,
        }
    }

    pub fn split(&mut self) -> Ant {
        let mut cloned = self.clone();
        self.energy /= 2;
        cloned.energy /= 2;
        cloned.age = 0;
        cloned
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    East,
    West,
    South,
    North,
}

impl Direction {
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        }
    }

    pub fn turn_right(self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }
}
