use std::io::Write;

use super::*;
use super::error::*;

pub struct Interpreter;

impl Interpreter {
    fn next_food(chunk: &mut Chunk, from_position: usize, previous_food: usize) -> Result<usize> {
        let mut min_distance = u32::MAX;
        let mut next_food = chunk.foods.len();
        for (index, food) in chunk.foods.iter().enumerate() {
            if previous_food == index {
                continue;
            }
            if chunk.eaten_foods[index] {
                continue;
            }
            if min_distance > food.distances[from_position] {
                min_distance = food.distances[from_position];
                next_food = index;
            }
        }
        if next_food != chunk.foods.len() {
            return Ok(next_food);
        }
        Err(Error::NoNextFood)
    }

    fn new_position(current: usize, direction: Tile, width: usize) -> Result<usize> {
        match direction {
            Tile::Right => Ok(current + 1),
            Tile::Down => Ok(current + width),
            Tile::Left => Ok(current - 1),
            Tile::Up => Ok(current - width),
            _ => Err(Error::TileNotDirection),
        }
    }

    fn input(bear: &mut Bear) -> Result<()> {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        let trim = buf.trim();
        if bear.collect_mode {
            let num: i32 = trim.parse().unwrap_or(0);
            bear.basket.push(num);
        } else {
            let mut v: Vec<i32> = trim.chars().map(|c| c as i32).collect();
            bear.basket.append(&mut v);
        }
        Ok(())
    }

    fn output(bear: &mut Bear) {
        if bear.collect_mode {
            print!("{}", bear.selected_value());
        } else if let Some(out) = bear.selected_char() {
            print!("{}", out);
        }
    }

    fn step_bear(chunk: &mut Chunk, bear_index: usize) -> Result<()> {
        let mut current_food = chunk.next_foods[bear_index];
        let bear_position = chunk.bears[bear_index].position;

        let should_find_new_food = current_food == chunk.foods.len()
            || chunk.foods[current_food].distances[bear_position] == u32::MAX;

        if should_find_new_food {
            // let op_next_food = Interpreter::next_food(chunk, bear_position, current_food);
            let next_food = Interpreter::next_food(chunk, bear_position, current_food)?;
            chunk.next_foods[bear_index] = next_food;
            current_food = next_food;
        }

        let mut new_position = Interpreter::new_position(
            bear_position,
            chunk.foods[current_food].directions[bear_position],
            chunk.width,
        )?;

        let mut new_tile = chunk.code[new_position];
        let bear_is_equal = chunk.bears[bear_index].is_equal();

        if matches!(new_tile, Tile::Gate) && !bear_is_equal {
            current_food = Interpreter::next_food(chunk, bear_position, current_food)?;
            chunk.next_foods[bear_index] = current_food;

            let changed_new_position = Interpreter::new_position(
                bear_position,
                chunk.foods[current_food].directions[bear_position],
                chunk.width,
            )?;
            
            if changed_new_position == new_position {
                return Err(Error::StuckAtGate);
            }

            new_position = changed_new_position;
            new_tile = chunk.code[new_position];
        }

        let bear = &mut chunk.bears[bear_index];
        match new_tile {
            Tile::Toggle => bear.toggle(),
            Tile::Single => {
                bear.food(false);
                chunk.eaten_foods[current_food] = true;
            },
            Tile::Many => bear.food(true),
            Tile::Input => Interpreter::input(bear)?,
            Tile::Output => Interpreter::output(bear),
            Tile::Shift => bear.shift(),
            Tile::Retrieve => bear.retrieve(),
            Tile::Append => bear.append(),
            Tile::Add => bear.add(),
            Tile::Mul => bear.mul(),
            Tile::And => bear.and(),
            Tile::Not => bear.not(),
            Tile::Swap => bear.swap(),
            _ => {}
        };

        bear.position = new_position;

        Ok(())
    }

    pub fn step(chunk: &mut Chunk) -> bool {
        let mut not_terminated = false;
        for i in 0..chunk.bears.len() {
            let result = Interpreter::step_bear(chunk, i);
            not_terminated |= result.is_ok();
            if let Err(error) = result {
                let msg = match error {
                    Error::Io(_) => "io error",
                    _ => "halted",
                };
                println!("\n| [{i}] bear: {}", msg);
            }
        }
        not_terminated
    }

    pub fn run(mut chunk: Chunk) {
        while Interpreter::step(&mut chunk) {}
    }

    pub fn step_through(mut chunk: Chunk) {
        let mut buf = String::new();
        loop {
            if !Interpreter::step(&mut chunk) {
                break;
            }

            println!("{}", chunk);
            print!("continue? [Y/n]: ");
            if std::io::stdout().flush().is_err() {
                break;
            }

            buf.clear();
            if std::io::stdin().read_line(&mut buf).is_err() {
                break;
            }
            let trim = buf.trim().to_lowercase();
            let should_continue = trim.is_empty() || trim == "y" || trim == "yes";

            if !should_continue {
                break;
            }
        }
    }
}
