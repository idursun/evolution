extern crate rand;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

const INSTRUCTION_DEFAULT_COUNT: usize = 20;
const DEFAULT_FOOD_ENERGY: usize = 110;
const DEFAULT_SPLIT_ENERGY: usize = 120;

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Ant {
    age: usize,
    energy: usize,
    current_index: usize,
    direction: Direction,
    gene: Gene,
}

impl Ant {
    fn new(current_index: usize) -> Ant {
        Ant {
            age: 0,
            current_index,
            energy: 50,
            direction: Direction::North,
            gene: Gene::random(),
        }
    }

    fn increase_energy(&mut self) {
        self.energy += DEFAULT_FOOD_ENERGY;
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
        self.age += 1;
        match self.gene.cycle() {
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
        }
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

    fn split(&mut self) -> Ant {
        let mut cloned = self.clone();
        self.energy = self.energy / 2;
        cloned.energy = cloned.energy / 2;
        cloned.age = 0;
        cloned
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
    Ant(Rc<RefCell<Ant>>),
}

struct Board {
    side: usize,
    cells: Vec<BoardCell>,
}

impl Board {
    fn new(side: usize) -> Board {
        let size = side * side;
        let mut rng = rand::thread_rng();
        let mut cells = Vec::with_capacity(size);

        for index in 0..=size {
            match rng.gen_range(0, 9) {
                0 => {
                    let ant = Rc::new(RefCell::new(Ant::new(index)));
                    cells.push(BoardCell::Ant(ant));
                }
                1 => cells.push(BoardCell::Food),
                _ => cells.push(BoardCell::Empty),
            }
        }

        Board { side, cells }
    }

    fn simulate(&mut self) {
        let ant_cells = self
            .cells
            .iter()
            .filter_map(|x| match x {
                BoardCell::Ant(ref cell) => Some(cell),
                _ => None,
            })
            .cloned()
            .collect::<Vec<_>>();

        //println!("ants left {}", ant_cells.len());
        for cell in ant_cells {
            let mut ant = cell.borrow_mut();
            ant.consume_energy();

            match ant.execute(self.side) {
                Some(Action::Move(ahead_index)) => {
                    if let BoardCell::Empty = self.cells[ahead_index] {
                        self.cells.swap(ahead_index, ant.current_index);
                        ant.current_index = ahead_index;
                        ant.consume_energy();
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

            if ant.energy > DEFAULT_SPLIT_ENERGY {
                if ant.current_index > 0 {
                    if let BoardCell::Empty = &self.cells[ant.current_index - 1] {
                        let mut born_ant = ant.split();
                        born_ant.current_index = ant.current_index - 1;
                        self.cells[ant.current_index - 1] =
                            BoardCell::Ant(Rc::new(RefCell::new(born_ant)));
                    }
                }
            }

            if ant.energy == 0 {
                //println!("was {:?}", &self.cells[ant.current_index]);
                self.cells[ant.current_index] = BoardCell::Food;
                //println!("removing {}", ant.current_index);
            }
        }
    }
}

fn print(board: &Board) {
    for (index, cell) in board.cells.iter().enumerate() {
        if index % board.side == 0 {
            println!();
        }
        match cell {
            BoardCell::Empty => print!(" "),
            BoardCell::Food => print!("."),
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
        //print(&board);
    }

    for cell in &board.cells {
        if let BoardCell::Ant(ref ant) = cell {
            let ant = ant.borrow();
            println!(
                "age: {}, current_index: {} - energy: {}: {:?}",
                ant.age, ant.current_index, ant.energy, ant.gene
            );
        }
    }
    print(&board);
}
