extern crate colored;
extern crate rand;

mod ant;
mod board;
mod gene;
mod team;

use board::{Board, BoardCell};
use colored::*;
use std::fmt::Write;
use std::thread;
use std::time::Duration;
use team::Team;

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
                    Team::Red if ant.energy > 100 => "@".black().on_red(),
                    Team::Red => "@".red(),
                    Team::Blue if ant.energy > 100 => "@".black().on_blue(),
                    Team::Blue => "@".blue(),
                    Team::Yellow if ant.energy > 100 => "@".black().on_yellow(),
                    Team::Yellow => "@".yellow(),
                    Team::Cyan if ant.energy > 100 => "@".black().on_magenta(),
                    Team::Cyan => "@".magenta(),
                };

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
        print(&board).unwrap();
        thread::sleep(Duration::from_millis(20));
    }

    for (index, cell) in board.cells.iter().enumerate() {
        if let BoardCell::Ant(ref ant) = cell {
            let ant = ant.borrow();
            println!(
                "* age: {}, current_index: {} - energy: {}: {:?}",
                ant.age, index, ant.energy, ant.gene
            );
        }
    }
    print(&board).unwrap();
}
