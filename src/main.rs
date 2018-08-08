extern crate rand;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

const INSTRUCTION_DEFAULT_COUNT: usize = 10;

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

impl Gene {
    fn cycle(&mut self) -> CellInstruction {
        let ret = self.code[self.instruction_pointer as usize];
        self.instruction_pointer = (self.instruction_pointer + 1) % (self.code.len() as u8);
        ret
    }
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
    Move(usize),
    Eat(usize),
}

#[derive(Debug)]
struct Ant {
    is_dead: bool,
    energy: usize,
    current_index: usize,
    direction: Direction,
    gene: Gene,
}

impl Ant {
    fn new(current_index: usize) -> Ant {
        Ant {
            is_dead: false,
            current_index,
            energy: 50,
            direction: Direction::North,
            gene: Gene::random(),
        }
    }

    fn increase_energy(&mut self) {
        self.energy += 100;
    }

    fn consume_energy(&mut self) {
        if self.energy > 0 {
            self.energy -= 1;
        }
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, self.gene.code.len());
        self.gene.code[index] = rand::random();
    }

    fn execute(&mut self, side: usize) -> Option<Action> {
        let current_op = self.gene.cycle();

        let action = match current_op {
            CellInstruction::TurnLeft => {
                self.direction = self.direction.turn_left();
                None
            }
            CellInstruction::TurnRight => {
                self.direction = self.direction.turn_right();
                None
            }
            CellInstruction::Move => if let Some(ahead_index) = self.ahead_index(side) {
                Some(Action::Move(ahead_index))
            } else {
                None
            },
            CellInstruction::Eat => if let Some(ahead_index) = self.ahead_index(side) {
                Some(Action::Eat(ahead_index))
            } else {
                None
            },
            _ => None,
        };

        action
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
    Ant(usize),
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
                    cells.push(BoardCell::Ant(ants.len()));
                }
                1 => cells.push(BoardCell::Food),
                _ => cells.push(BoardCell::Empty),
            }
        }

        Board { side, cells, ants }
    }

    fn simulate(&mut self) {
        //let mut dead_ants: Vec<&Ant> = Vec::new();
        for ant in &mut self.ants {
            if ant.is_dead {
                continue;
            }
            ant.consume_energy();

            match ant.execute(self.side) {
                Some(Action::Move(ahead_index)) => {
                    if let BoardCell::Empty = self.cells[ahead_index] {
                        self.cells.swap(ahead_index, ant.current_index);
                        ant.current_index = ahead_index;
                        ant.consume_energy();

                        if ant.energy == 0 {
                            self.cells[ant.current_index] = BoardCell::Food;
                            ant.is_dead = true;
                            //dead_ants.push(ant);
                        }
                    }
                }
                Some(Action::Eat(ahead_index)) => {
                    ant.consume_energy();
                    if let BoardCell::Food = self.cells[ahead_index] {
                        self.cells[ahead_index] = BoardCell::Empty;
                        ant.increase_energy();
                    }
                }
                None => {
                    ant.mutate();
                }
            }
        }

        //for dead_ant in dead_ants {
        //    let index = self
        //        .ants
        //        .iter_mut()
        //        .position(|x| x.current_index == dead_ant.current_index)
        //        .unwrap();

        //    self.ants.remove(index);
        //}
    }
}

fn print(board: &Board) {
    for (index, cell) in board.cells.iter().enumerate() {
        if index % board.side == 0 {
            println!();
        }
        match cell {
            BoardCell::Empty => print!("."),
            BoardCell::Food => print!("o"),
            BoardCell::Ant(_) => print!("@"),
        }
    }
}

fn main() {
    let mut board = Board::new(30);
    let mut count = 1000;
    while count > 0 {
        board.simulate();
        count -= 1;
        print(&board);
    }

    for ant in &board.ants {
        if !ant.is_dead {
            println!(
                "{} current_index: {} - energy: {}: {:?}",
                ant.is_dead, ant.current_index, ant.energy, ant.gene
            );
        }
    }
    print(&board);
}
