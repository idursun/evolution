extern crate rand;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

const INSTRUCTION_DEFAULT_COUNT: usize = 200;

#[derive(Debug, Copy, Clone)]
enum CellInstruction {
    Noop,
    Move,
    TurnLeft,
    TurnRight,
    Eat,
}

impl Distribution<CellInstruction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CellInstruction {
        match rng.gen_range(0, 5) {
            0 => CellInstruction::Noop,
            1 => CellInstruction::Move,
            2 => CellInstruction::TurnLeft,
            3 => CellInstruction::TurnRight,
            4 => CellInstruction::Eat,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Gene {
    instruction_pointer: u8,
    cycle_limit: u8,
    code: Vec<CellInstruction>,
}

#[derive(Debug, Clone)]
enum Direction {
    East,
    West,
    South,
    North,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }
}

#[derive(Debug)]
enum Action {
    Move,
    Eat,
}

#[derive(Debug)]
struct Ant {
    energy: usize,
    current_index: usize,
    direction: Direction,
    gene: Gene,
    action: Option<Action>,
}

impl Ant {
    fn new(current_index: usize) -> Ant {
        Ant {
            current_index,
            energy: 0,
            direction: Direction::North,
            gene: Gene::random(),
            action: None,
        }
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, self.gene.code.len());
        self.gene.code[index] = rand::random();
    }

    fn execute(&mut self) {
        let current_op = &self.gene.code[self.gene.instruction_pointer as usize];
        self.action = None;

        match current_op {
            CellInstruction::TurnLeft => {
                self.direction = self.direction.turn_left();
            }
            CellInstruction::TurnRight => {
                self.direction = self.direction.turn_right();
            }
            CellInstruction::Move => self.action = Some(Action::Move),
            CellInstruction::Eat => self.action = Some(Action::Eat),
            _ => {}
        };

        self.gene.instruction_pointer =
            (self.gene.instruction_pointer + 1) % (self.gene.code.len() as u8);
    }

    fn ahead_index(&self, size: usize) -> Option<usize> {
        let board_size = size * size;
        match self.direction {
            Direction::East => if self.current_index + 1 < board_size {
                Some(self.current_index + 1)
            } else {
                None
            },
            Direction::West => if self.current_index > 0 {
                Some(self.current_index - 1)
            } else {
                None
            },
            Direction::North => if self.current_index >= size {
                Some(self.current_index - size)
            } else {
                None
            },
            Direction::South => if self.current_index + size < board_size {
                Some(self.current_index + size)
            } else {
                None
            },
        }
    }
}

impl Gene {
    fn new() -> Gene {
        Gene {
            instruction_pointer: 0,
            cycle_limit: 2,
            code: vec![],
        }
    }

    fn random() -> Gene {
        let mut gene = Gene::new();

        for _ in 0..INSTRUCTION_DEFAULT_COUNT {
            gene.code.push(rand::random());
        }

        gene
    }
}

#[derive(Debug)]
enum BoardCell {
    Empty,
    Food,
    HasAnt(usize),
}

struct Board {
    side: usize,
    cells: Vec<BoardCell>,
    ants: Vec<Ant>,
}

impl Board {
    fn new(side: usize) -> Board {
        let size = side * side;
        let mut rng = rand::thread_rng();
        let mut ants = Vec::new();
        let mut cells = Vec::with_capacity(size);

        for index in 0..=size {
            match rng.gen_range(0, 9) {
                0 => {
                    ants.push(Ant::new(index));
                    cells.push(BoardCell::HasAnt(ants.len()));
                }
                1 => cells.push(BoardCell::Food),
                _ => cells.push(BoardCell::Empty),
            }
        }

        Board {
            side,
            cells: cells,
            ants: ants,
        }
    }

    fn simulate(&mut self) {
        for ant in self.ants.iter_mut() {
            ant.execute();
            if let Some(ahead_index) = ant.ahead_index(self.side) {
                match ant.action {
                    Some(Action::Move) => match self.cells[ahead_index] {
                        BoardCell::Empty => {
                            self.cells.swap(ahead_index, ant.current_index);
                            //println!("moved {} to {}", ant.current_index, ahead_index);
                            ant.current_index = ahead_index;
                        }
                        _ => {}
                    },
                    Some(Action::Eat) => match self.cells[ahead_index] {
                        BoardCell::Food => {
                            self.cells[ahead_index] = BoardCell::Empty;
                            //println!("consumed food at {}", ahead_index);
                            ant.energy += 1;
                        }
                        _ => {
                            ant.mutate();
                        }
                    },
                    None => {}
                }
            }
        }
    }
}

fn main() {
    let mut board = Board::new(30);
    let mut count = 1000;
    while count > 0 {
        board.simulate();
        count -= 1;
    }

    for ant in board.ants {
        println!(
            "current_index: {} - energy: {}",
            ant.current_index, ant.energy
        );
    }

    for (index, cell) in board.cells.iter().enumerate() {
        if index % board.side == 0 {
            println!();
        }
        match cell {
            BoardCell::Empty => print!("."),
            BoardCell::Food => print!("o"),
            BoardCell::HasAnt(_) => print!("X"),
        }
    }
}
