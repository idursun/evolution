extern crate colored;
extern crate rand;
use colored::*;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;
use std::cell::RefCell;
use std::fmt::Write;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

const INSTRUCTION_DEFAULT_COUNT: usize = 20;
const DEFAULT_ANT_ENERGY: i32 = 50;
const DEFAULT_FOOD_ENERGY: i32 = 150;
const DEFAULT_SPLIT_ENERGY: i32 = 120;

#[derive(Debug, Copy, Clone)]
enum CellInstruction {
    Noop,
    Move,
    TurnLeft,
    TurnRight,
    Eat,
    Attack,
    JmpNe(u8),
    Jmp(u8),
}

impl Distribution<CellInstruction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CellInstruction {
        match rng.gen_range(0, 8) {
            0 => CellInstruction::Noop,
            1 => CellInstruction::Move,
            2 => CellInstruction::TurnLeft,
            3 => CellInstruction::TurnRight,
            4 => CellInstruction::Eat,
            5 => CellInstruction::Attack,
            6 => CellInstruction::Jmp(rng.gen_range(0, INSTRUCTION_DEFAULT_COUNT as u8)),
            7 => CellInstruction::JmpNe(rng.gen_range(0, INSTRUCTION_DEFAULT_COUNT as u8)),
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
    fn new() -> Gene {
        Gene {
            instruction_pointer: 0,
            cycle_limit: 2,
            code: vec![],
        }
    }

    fn random() -> Gene {
        let mut gene = Gene::new();

        //gene.code.push(CellInstruction::Eat);
        //gene.code.push(CellInstruction::Jmp(0));
        //gene.code.push(CellInstruction::Move);
        //gene.code.push(CellInstruction::Jmp(0));
        ////gene.code.push(CellInstruction::TurnLeft);
        //gene.code.push(CellInstruction::Jmp(0));
        //gene.code.push(CellInstruction::JmpNe(2));
        while gene.code.len() < INSTRUCTION_DEFAULT_COUNT {
            gene.code.push(rand::random());
        }

        gene
    }

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
    Attack(usize),
}

#[derive(Debug, Clone, PartialEq)]
enum Team {
    Blue,
    Red,
    Yellow,
    Cyan,
}

#[derive(Debug, Clone)]
struct Ant {
    sensor: bool,
    team: Team,
    age: usize,
    energy: i32,
    current_index: usize,
    direction: Direction,
    gene: Gene,
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

impl Ant {
    fn new(current_index: usize) -> Ant {
        Ant {
            sensor: false,
            team: rand::random(),
            age: 0,
            current_index,
            energy: DEFAULT_ANT_ENERGY,
            direction: Direction::North,
            gene: Gene::random(),
        }
    }

    fn increase_energy(&mut self) {
        self.energy += DEFAULT_FOOD_ENERGY;
    }

    fn consume_energy(&mut self, amount: i32) {
        self.energy -= amount;
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0, self.gene.code.len());
        self.gene.code[index] = rand::random();
    }

    fn execute(&mut self, width: usize, height: usize) -> Option<Action> {
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
            CellInstruction::Attack => self.ahead_index(width, height).map(Action::Attack),
            CellInstruction::Move => self.ahead_index(width, height).map(Action::Move),
            CellInstruction::Eat => self.ahead_index(width, height).map(Action::Eat),
            _ => None,
        }
    }

    fn ahead_index(&self, width: usize, height: usize) -> Option<usize> {
        let y = self.current_index / width;
        let x = self.current_index % width;

        let next_pos = match self.direction {
            Direction::East if x + 1 < width => Some((x + 1, y)),
            Direction::West if x > 0 => Some((x - 1, y)),
            Direction::North if y > 0 => Some((x, y - 1)),
            Direction::South if y + 1 < height => Some((x, y + 1)),
            _ => None,
        };

        next_pos.map(|(x, y)| x + y * width)
    }

    fn split(&mut self) -> Ant {
        let mut cloned = self.clone();
        self.energy /= 2;
        cloned.energy /= 2;
        cloned.age = 0;
        cloned
    }
}

#[derive(Debug)]
enum BoardCell {
    Empty,
    Food,
    Ant(Rc<RefCell<Ant>>),
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<BoardCell>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let size = width * height;
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

        Board {
            width,
            height,
            cells,
        }
    }

    fn around(&self, index: usize) -> [bool; 4] {
        let mut ret = [false; 4];
        if index > 0 {
            ret[0] = match self.cells[index - 1] {
                BoardCell::Empty => false,
                _ => false,
            }
        }
        if index + 1 < self.cells.len() {
            ret[1] = match self.cells[index + 1] {
                BoardCell::Empty => false,
                _ => false,
            }
        }
        if index + self.width < self.cells.len() {
            ret[2] = match self.cells[index + self.width] {
                BoardCell::Empty => false,
                _ => false,
            }
        }
        if index > self.width {
            ret[3] = match self.cells[index - self.width] {
                BoardCell::Empty => false,
                _ => false,
            }
        }
        ret
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
            ant.consume_energy(1);

            match ant.execute(self.width, self.height) {
                Some(Action::Move(ahead_index)) => {
                    if let BoardCell::Empty = self.cells[ahead_index] {
                        self.cells.swap(ahead_index, ant.current_index);
                        ant.current_index = ahead_index;
                        ant.consume_energy(1);
                        let around = self.around(ant.current_index);
                        let is_around = around.iter().any(|&x| x);
                        ant.sensor = is_around;
                    }
                }
                Some(Action::Attack(ahead_index)) => {
                    ant.consume_energy(1);
                    if let BoardCell::Ant(ref ahead_ant) = self.cells[ahead_index] {
                        let mut ahead_ant = ahead_ant.borrow_mut();
                        if ahead_ant.team != ant.team {
                            ahead_ant.consume_energy(ant.energy / 10);
                        }
                    }
                }
                Some(Action::Eat(ahead_index)) => {
                    ant.consume_energy(1);
                    if let BoardCell::Food = self.cells[ahead_index] {
                        self.cells[ahead_index] = BoardCell::Empty;
                        ant.increase_energy();
                    }
                }
                None => {
                    if ant.age % 10 == 0 {
                        ant.mutate();
                    }
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

            if ant.energy <= 0 {
                self.cells[ant.current_index] = BoardCell::Food;
            }
        }
    }
}

fn print(board: &Board) -> std::result::Result<(), std::fmt::Error> {
    let mut buffer = String::new();
    write!(&mut buffer, "{}[2J", 27 as char)?;
    for (index, cell) in board.cells.iter().enumerate() {
        if index % board.width == 0 {
            writeln!(&mut buffer)?;
        }
        match cell {
            BoardCell::Empty => write!(&mut buffer, ".")?,
            BoardCell::Food => write!(&mut buffer, "{}", "x".green())?,
            BoardCell::Ant(ref ant) => {
                let ant = ant.borrow();
                let mut text = match ant.team {
                    Team::Red => "@".red(),
                    Team::Blue => "@".blue(),
                    Team::Yellow => "@".yellow(),
                    Team::Cyan => "@".magenta(),
                };

                if ant.energy > 100 {
                    text = text.on_green();
                }

                write!(&mut buffer, "{}", text)?;
            }
        }
    }
    println!("{}", buffer);
    Result::Ok(())
}

fn main() {
    let mut board = Board::new(50, 50);
    let mut count = 10000;
    while count > 0 {
        board.simulate();
        count -= 1;
        //println!("{}", count);
        print(&board).unwrap();
        thread::sleep(Duration::from_millis(20));
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
    print(&board).unwrap();
}
