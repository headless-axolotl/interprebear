use std::fmt::Display;

#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    Empty,
    Bear,
    // One way doors
    Right,
    Down,
    Left,
    Up,
    // Conditional gate
    Gate,
    // Blocked tile
    Tree,
    // Collect mode toggle
    Toggle,
    // Food
    Single,
    Many,
    Input,
    Output,
    // Shift selected cell
    Shift,
    // Also encode the opposite operation when bear is in collect mode
    Retrieve,
    Append,
    Add,
    Mul,
    And,
    Not,
    Swap,
    None,
}

impl Tile {
    pub fn is_food(&self) -> bool {
        matches!(self, Single | Many)
    }

    pub fn is_direction(&self) -> bool {
        matches!(self, Right | Down | Left | Up)
    }

    pub fn opposite_direction(&self) -> Tile {
        match *self {
            Right => Left,
            Down => Up,
            Left => Right,
            Up => Down,
            _ => Empty,
        }
    }

    pub fn can_move_to(&self, to: Tile) -> bool {
        if matches!(to, Tree) { return false; }
        if *self == to { return false; }
        true
    }
}

use Tile::*;
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Empty => '.',
            Bear => '#',
            Right => '>',
            Down => '_',
            Left => '<',
            Up => '^',
            Gate => ':',
            Tree => '|',
            Toggle => '~',
            Single => '\'',
            Many => '@',
            Input => '?',
            Output => '!',
            Shift => '\"',
            Retrieve => '=',
            Append => ';',
            Add => '+',
            Mul => '*',
            And => '&',
            Not => '-',
            Swap => '%',
            None => ' ',
        };
        if matches!(self, None) {
            return Ok(());
        }
        write!(f, "{}", display)
    }
}
