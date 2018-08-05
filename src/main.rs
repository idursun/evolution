extern crate rand;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;

const INSTRUCTION_DEFAULT_COUNT: usize = 20;

#[derive(Debug)]
enum Instruction {
    Noop,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

impl Distribution<Instruction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Instruction {
        match rng.gen_range(0, 5) {
            0 => Instruction::Noop,
            1 => Instruction::MoveUp,
            2 => Instruction::MoveDown,
            3 => Instruction::MoveLeft,
            4 => Instruction::MoveRight,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Gene {
    instruction_pointer: u8,
    cycle_limit: u8,
    code: Vec<Instruction>,
}

#[derive(Debug)]
struct Ant {
    x: usize,
    y: usize,
    energy: usize,
    gene: Gene,
}

impl Ant {
    fn new(x: usize, y: usize) -> Ant {
        Ant {
            x,
            y,
            energy: 0,
            gene: Gene::random(),
        }
    }

    //    fn execute_gene(&mut self, board: &Board) {
    //        let current_op = &self.gene.code[0];
    //
    //        match current_op {
    //            Instruction::Noop => {}
    //            Instruction::MoveUp => self.y -= 1,
    //            Instruction::MoveDown => self.y += 1,
    //            Instruction::MoveLeft => self.x -= 1,
    //            Instruction::MoveRight => self.x += 1,
    //        }
    //
    //        self.gene.instruction_pointer += 1;
    //    }
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
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1, INSTRUCTION_DEFAULT_COUNT);

        for _ in 0..r {
            gene.code.push(rand::random());
        }

        gene
    }
}

enum BoardCell {
    Empty,
    Food,
    HasAnt(Ant),
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<BoardCell>,
    ants: Vec<Ant>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let size = width * height;
        let mut rng = rand::thread_rng();

        let count = rng.gen_range(0, size);
        let mut ants = Vec::with_capacity(count);
        for _ in 0..count {
            let rx = rng.gen_range(0, width);
            let ry = rng.gen_range(0, height);
            ants.push(Ant::new(rx, ry));
        }

        let mut cells = Vec::with_capacity(size);
        for _ in 0..size {
            cells.push(BoardCell::Empty);
        }

        Board {
            width,
            height,
            ants,
            cells: cells,
        }
    }

    fn simulate(&mut self) {
        for ant in self.ants.iter_mut() {
            let current_op = &ant.gene.code[ant.gene.instruction_pointer as usize];

            match current_op {
                Instruction::Noop => {}
                Instruction::MoveUp => {
                    if ant.y > 0 {
                        ant.y -= 1;
                    }
                }
                Instruction::MoveDown => {
                    if ant.y + 1 < self.height {
                        ant.y += 1;
                    }
                }
                Instruction::MoveLeft => {
                    if ant.x > 0 {
                        ant.x -= 1;
                    }
                }
                Instruction::MoveRight => {
                    if ant.x + 1 < self.width {
                        ant.x += 1;
                    }
                }
            }

            ant.gene.instruction_pointer += 1;
        }
    }
}

fn main() {
    let mut board = Board::new(16, 16);
    println!("{}", board.ants.len());
    board.simulate();
    for ant in board.ants {
        println!("{:?}", ant);
    }
}
