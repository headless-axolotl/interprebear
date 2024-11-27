use std::{collections::VecDeque, fmt::Display};

use super::*;

pub struct Chunk {
    pub code: Vec<Tile>,
    pub bears: Vec<Bear>,
    pub next_foods: Vec<usize>,
    pub eaten_foods: Vec<bool>,
    pub foods: Vec<Food>,
    pub width: usize,
}

impl Chunk {
    pub fn block(length: usize) -> Vec<Tile> {
        vec![Tile::Empty; length]
    }
    
    pub fn print(code: &[Tile], width: usize) {
        for (index, tile) in code.iter().enumerate() {
            print!("{}", tile);
            if index % width == width - 1 {
                println!();
            }
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut duplicate = self.code.clone();
        for bear in &self.bears {
            duplicate[bear.position] = Tile::Bear;
        }
        for (index, tile) in duplicate.iter().enumerate() {
            write!(f, "{}", tile)?;
            if index % self.width == self.width - 1 {
                writeln!(f)?;
            }
        }

        writeln!(f)?;
        
        for (index, bear) in self.bears.iter().enumerate() {
            writeln!(f, "| [{}] bear: {} ({})", index, bear.value, bear.swap)?;
            
            let basket_str = bear.basket
                .iter()
                .enumerate()
                .map(|item| if item.0 != bear.selected { item.1.to_string() }
                            else { format!("[{}]", item.1) })
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(f, "| basket: [{}]", basket_str)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Food {
    pub distances: Vec<u32>,
    pub directions: Vec<Tile>,
}

impl Food {
    pub fn generate(code: &[Tile], width: usize, position: usize) -> Food {
        let len = code.len();
        let mut distances = vec![u32::MAX; len];
        let mut directions = vec![Tile::Empty; len];
 
        let mut queue = VecDeque::new();
        
        distances[position] = 0;
        queue.push_back(position);

        while let Some(current) = queue.pop_front() {

            let new_distance = distances[current] + 1;
            
            let mut update_neighbour = |neighbour: usize, direction: Tile| {
                if !direction.can_move_to(code[neighbour]) { return; }
                if new_distance > distances[neighbour] { return; }
                if new_distance < distances[neighbour] {
                    distances[neighbour] = new_distance;
                    directions[neighbour] = direction.opposite_direction();
                    queue.push_back(neighbour);
                } else {
                    directions[neighbour] = directions[neighbour].min(
                        direction.opposite_direction()
                    );
                }
            };

            let right = current + 1;
            if right % width != 0 {
                update_neighbour(right, Tile::Right);
            }

            let down = current + width;
            if down < len {
                update_neighbour(down, Tile::Down);
            }

            if current % width != 0 {
                update_neighbour(current - 1, Tile::Left);
            }

            if current > width {
                update_neighbour(current - width, Tile::Up);
            }
        }
        
        distances[position] = u32::MAX;

        Food { distances, directions }
    }
}
