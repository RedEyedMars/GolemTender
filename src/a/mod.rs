use failure::err_msg;

pub mod b;
pub mod benny;
pub mod e;
mod input;

use crate::w::g::res::Resources;
use std::time::Instant;

use b::Board;
use e::gol::Golem;

//use crate::packed_simd::f32x4;
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum EntityType {
    Wall,    // |_
    Golem,   // _|
    Block,   // |``
    Door,    // ``|
    Switch,  // | |
    Nothing, // =
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Direction {
    East,  // `+
    West,  // `x
    North, // `^
    South, // `v
}

use packed_simd::u32x2;
impl Direction {
    pub fn advance(&self, xy: u32x2, width: u32, height: u32) -> Result<u32x2, ()> {
        use Direction::*;
        match self {
            East if xy.extract(0) + 1 < width => Ok(xy + u32x2::new(1u32, 0u32)),
            West if xy.extract(0) > 0 => Ok(xy - u32x2::new(1u32, 0u32)),
            North if xy.extract(1) + 1 < height => Ok(xy + u32x2::new(0u32, 1u32)),
            South if xy.extract(1) < height => Ok(xy - u32x2::new(0u32, 1u32)),
            _ => Err(()),
        }
    }
    pub fn retreat(&self, xy: u32x2, width: u32, height: u32) -> Result<u32x2, ()> {
        use Direction::*;
        match self {
            East if xy.extract(0) > 0 => Ok(xy - u32x2::new(1u32, 032)),
            West if xy.extract(0) + 1 < width => Ok(xy + u32x2::new(1u32, 032)),
            South if xy.extract(1) + 1 < height => Ok(xy + u32x2::new(0u32, 132)),
            North if xy.extract(1) < height => Ok(xy - u32x2::new(0u32, 132)),
            _ => Err(()),
        }
    }
    pub fn turn_dex(self) -> Direction {
        use Direction::*;
        match self {
            East => South,
            South => West,
            West => North,
            North => East,
        }
    }
    pub fn turn_sin(self) -> Direction {
        use Direction::*;
        match self {
            East => North,
            South => East,
            West => South,
            North => West,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Colour {
    Red,    // +
    Green,  // x
    Blue,   // ^
    Purple, // v
}

impl Colour {
    pub fn from_varying(c: char) -> Colour {
        match c {
            'P' => Colour::Purple,
            'R' => Colour::Red,
            'B' => Colour::Blue,
            'G' => Colour::Green,
            _ => Colour::Purple,
        }
    }
    pub fn to_varying(&self) -> char {
        match self {
            Colour::Blue => 'B',
            Colour::Red => 'R',
            Colour::Green => 'G',
            Colour::Purple => 'P',
        }
    }
}

pub struct GameState {
    //pub clock: Instant,
    pub board: Board,
    pub board_index: usize,
    pub boards: Vec<Board>,
    pub golems: Vec<Golem>,
    pub animation_state: u8,
    pub res: Resources,
}

pub fn setup(context: crate::w::g::Context) -> Result<GameState, failure::Error> {
    let mut res = context.assets();
    let board = Board::new(String::from("Starting Grove"), 8usize, 6usize, &mut res)?; // this will be the default Board::new(86usize, 64usize, &mut res)?;

    Ok(GameState {
        //clock: Instant::now(),
        board: board.clone(),
        board_index: 0usize,
        boards: vec![board],
        golems: Vec::new(),
        animation_state: 0u8,
        res,
    })
}

pub fn run(game: &mut GameState) -> Result<bool, failure::Error> {
    if input::detect_input(game)? {
        return Ok(false);
    }

    //game.animation_state = (game.clock.elapsed().as_millis() / 400 % 2) as u8;
    game.board.execute()?;
    game.board.render(&game)?;
    //
    Ok(true)
}
