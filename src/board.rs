extern crate rand;
use ant::{Action, Ant, Direction};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

const DEFAULT_SPLIT_ENERGY: i32 = 120;

#[derive(Debug)]
pub enum BoardCell {
    Empty,
    Food,
    Ant(Rc<RefCell<Ant>>),
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<BoardCell>,
}

pub enum BoardMutation {
    Food(usize),
    Remove(usize),
    Born(usize, Rc<RefCell<Ant>>),
    Swap(usize, usize),
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let size = width * height;
        let mut rng = rand::thread_rng();
        let mut cells = Vec::with_capacity(size);

        for _ in 0..=size {
            match rng.gen_range(0, 9) {
                0 => {
                    let ant = Rc::new(RefCell::new(Ant::new()));
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

    pub fn around(&self, index: usize) -> bool {
        let around_indices = [
            index as i32 - 1,
            index as i32 + 1,
            index as i32 - self.width as i32,
            index as i32 + self.width as i32,
        ];

        let size = (self.width * self.height) as i32;

        around_indices
            .iter()
            .filter(|&x| *x < size && *x > 0)
            .any(|x| match self.cells[*x as usize] {
                BoardCell::Empty => true,
                _ => false,
            })
    }

    pub fn ahead_index(&self, index: usize, direction: Direction) -> Option<usize> {
        let y = index / self.width;
        let x = index % self.width;

        let next_pos = match direction {
            Direction::East if x + 1 < self.width => Some((x + 1, y)),
            Direction::West if x > 0 => Some((x - 1, y)),
            Direction::North if y > 0 => Some((x, y - 1)),
            Direction::South if y + 1 < self.height => Some((x, y + 1)),
            _ => None,
        };

        next_pos.map(|(x, y)| x + y * self.width)
    }

    pub fn simulate(&mut self) {
        let mut mutations: Vec<BoardMutation> = Vec::new();
        for (index, boardcell) in self.cells.iter().enumerate() {
            match boardcell {
                BoardCell::Empty => {
                    let maybe_food: f64 = rand::random::<f64>();
                    if maybe_food < 0.001 {
                        mutations.push(BoardMutation::Food(index));
                    }
                }
                BoardCell::Food => continue,
                BoardCell::Ant(ref cell) => {
                    let mut ant = cell.borrow_mut();
                    let ahead_index = self.ahead_index(index, ant.direction);

                    match ant.execute() {
                        Some(Action::Move) => {
                            if let Some(ahead_index) = ahead_index {
                                if let BoardCell::Empty = self.cells[ahead_index] {
                                    mutations.push(BoardMutation::Swap(ahead_index, index));
                                    ant.consume_energy(1);
                                    ant.sensor = self.around(index);
                                }
                            }
                        }
                        Some(Action::Attack) => {
                            ant.consume_energy(1);
                            if let Some(ahead_index) = ahead_index {
                                if let BoardCell::Ant(ref ahead_ant) = self.cells[ahead_index] {
                                    let mut ahead_ant = ahead_ant.borrow_mut();
                                    if ahead_ant.team != ant.team {
                                        ahead_ant.consume_energy(ant.energy / 10);
                                    }
                                }
                            }
                        }
                        Some(Action::Eat) => {
                            ant.consume_energy(1);
                            if let Some(ahead_index) = ahead_index {
                                if let BoardCell::Food = self.cells[ahead_index] {
                                    mutations.push(BoardMutation::Remove(ahead_index));
                                    ant.increase_energy();
                                }
                            }
                        }
                        None => {
                            ant.mutate();
                        }
                    }

                    if ant.energy > DEFAULT_SPLIT_ENERGY {
                        if index > 0 {
                            if let BoardCell::Empty = &self.cells[index - 1] {
                                let mut born_ant = ant.split();
                                mutations.push(BoardMutation::Born(
                                    index - 1,
                                    Rc::new(RefCell::new(born_ant)),
                                ));
                            }
                        }
                    }

                    if ant.energy <= 0 {
                        mutations.push(BoardMutation::Food(index));
                    }
                }
            }
        }

        for mutation in mutations {
            match mutation {
                BoardMutation::Food(index) => self.cells[index] = BoardCell::Food,
                BoardMutation::Remove(index) => self.cells[index] = BoardCell::Empty,
                BoardMutation::Swap(from, to) => self.cells.swap(from, to),
                BoardMutation::Born(index, born) => {
                    self.cells[index] = BoardCell::Ant(Rc::clone(&born));
                }
            }
        }
    }
}
