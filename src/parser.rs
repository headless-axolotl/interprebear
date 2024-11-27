use super::*;

pub struct Parser;

impl Parser {
    pub fn parse(buf: &str) -> Chunk {
        let mut width = 0usize;
        let lines: Vec<&str> = buf.lines().collect();
        for line in &lines {
            width = width.max(line.len());
        }

        let tile_count = width * lines.len();
        let mut code = vec![Tile::Empty; tile_count];
        let mut index = 0usize;

        let mut food_positions = vec![];
        let mut bear_positions = vec![];

        for line in &lines {
            for char_tile in line.trim().chars() {
                let tile = Parser::parse_char(char_tile);
                
                if matches!(tile, Tile::None) {
                    continue;
                }
                
                if matches!(tile, Tile::Bear) {
                    bear_positions.push(index);
                    index += 1;
                    continue;
                }

                if tile.is_food() {
                    food_positions.push(index);
                }

                code[index] = tile;
                index += 1;
            }
            if index % width != 0 {
                index += width - (index % width);
            }
        }

        let food_count = food_positions.len();
        let bear_count = bear_positions.len();

        let mut foods = vec![];
        for position in food_positions {
            foods.push(Food::generate(&code, width, position));
        }
        
        let mut bears = vec![];
        for position in bear_positions {
            bears.push(Bear::new(position));
        }

        // for food in &foods {
        //     Chunk::print(&food.directions, width);
        //     println!();
        // }

        Chunk {
            code,
            bears,
            next_foods: vec![food_count; bear_count],
            eaten_foods: vec![false; food_count],
            foods,
            width,
        }
    }

    // In the future will be read from a config file. Default: ascii_bear.
    pub fn parse_char(tile: char) -> Tile {
        match tile {
            '.' => Tile::Empty,
            '#' => Tile::Bear,
            '>' => Tile::Right,
            '_' => Tile::Down,
            '<' => Tile::Left,
            '^' => Tile::Up,
            ':' => Tile::Gate,
            '|' => Tile::Tree,
            '~' => Tile::Toggle,
            '\'' => Tile::Single,
            '@' => Tile::Many,
            '?' => Tile::Input,
            '!' => Tile::Output,
            '\"' => Tile::Shift,
            '=' => Tile::Retrieve,
            ';' => Tile::Append,
            '+' => Tile::Add,
            '*' => Tile::Mul,
            '&' => Tile::And,
            '-' => Tile::Not,
            '%' => Tile::Swap,
            _ => Tile::None,
        }
    }
}
