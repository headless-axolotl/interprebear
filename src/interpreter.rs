use std::io::Write;

use super::*;

pub struct Interpreter;

impl Interpreter {
    fn next_food(chunk: &mut Chunk, from_position: usize, previous_food: usize) -> Option<usize> {
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
            return Some(next_food);
        }
        None
    }

    fn new_position(current: usize, direction: Tile, width: usize) -> Option<usize> {
        match direction {
            Tile::Right => Some(current + 1),
            Tile::Down => Some(current + width),
            Tile::Left => Some(current - 1),
            Tile::Up => Some(current - width),
            _ => None,
        }
    }

    fn input(bear: &mut Bear) -> bool {
        let mut buf = String::new();
        if std::io::stdin().read_line(&mut buf).is_err() {
            return false;
        }
        let trim = buf.trim();
        if bear.collect_mode {
            let num: i32 = trim.parse().unwrap_or(0);
            bear.basket.push(num);
        } else {
            let mut v: Vec<i32> = trim.chars().map(|c| c as i32).collect();
            bear.basket.append(&mut v);
        }
        true
    }

    fn output(bear: &mut Bear) {
        if bear.collect_mode {
            print!("{}", bear.selected_value());
        } else if let Some(out) = bear.selected_char() {
            print!("{}", out);
        }
    }

    fn step_bear(chunk: &mut Chunk, bear_index: usize) -> bool {
        let mut current_food = chunk.next_foods[bear_index];
        let bear_position = chunk.bears[bear_index].position;

        let should_find_new_food = current_food == chunk.foods.len()
            || chunk.foods[current_food].distances[bear_position] == u32::MAX;

        if should_find_new_food {
            let op_next_food = Interpreter::next_food(chunk, bear_position, current_food);
            if let Some(next_food) = op_next_food {
                chunk.next_foods[bear_index] = next_food;
                current_food = next_food;
            } else {
                return false;
            }
        }

        let op_new_position = Interpreter::new_position(
            bear_position,
            chunk.foods[current_food].directions[bear_position],
            chunk.width,
        );
        let mut new_position = match op_new_position {
            Some(value) => value,
            None => return false,
        };

        let mut new_tile = chunk.code[new_position];
        let bear_is_equal = chunk.bears[bear_index].is_equal();

        if matches!(new_tile, Tile::Gate) && !bear_is_equal {
            let op_next_food = Interpreter::next_food(chunk, bear_position, current_food);
            if op_next_food.is_none() {
                return false;
            }

            current_food = op_next_food.unwrap();
            chunk.next_foods[bear_index] = current_food;

            let op_new_position = Interpreter::new_position(
                bear_position,
                chunk.foods[current_food].directions[bear_position],
                chunk.width,
            );
            let changed_new_position = match op_new_position {
                Some(value) => value,
                None => return false,
            };

            if changed_new_position == new_position {
                return false;
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
            Tile::Input => {
                if !Interpreter::input(bear) {
                    return false;
                }
            }
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

        true
    }

    pub fn step(chunk: &mut Chunk) -> bool {
        let mut not_terminated = false;
        for i in 0..chunk.bears.len() {
            not_terminated |= Interpreter::step_bear(chunk, i);
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
