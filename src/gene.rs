extern crate rand;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

const INSTRUCTION_DEFAULT_COUNT: usize = 20;

#[derive(Debug, Copy, Clone)]
pub enum CellInstruction {
    Noop,
    Move,
    TurnLeft,
    TurnRight,
    Eat,
    Attack,
    JmpNe(u8),
    Jmp(u8),
}

#[derive(Debug, Clone)]
pub struct Gene {
    pub instruction_pointer: u8,
    pub cycle_limit: u8,
    pub code: Vec<CellInstruction>,
}

impl Gene {
    pub fn new() -> Gene {
        Gene {
            instruction_pointer: 0,
            cycle_limit: 2,
            code: vec![],
        }
    }

    pub fn random() -> Gene {
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

    pub fn cycle(&mut self) -> CellInstruction {
        let ret = self.code[self.instruction_pointer as usize];
        self.instruction_pointer = (self.instruction_pointer + 1) % (self.code.len() as u8);
        ret
    }
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
