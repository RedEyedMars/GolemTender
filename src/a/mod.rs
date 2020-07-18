use failure::err_msg;

pub mod b;
pub mod benny;
pub mod e;
mod input;

use crate::g::render_gl::Viewport;
use crate::g::resources::Resources;
use sdl2::EventPump;
use sdl2::Sdl;
use std::time::Instant;

use b::Board;

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

pub struct GameState {
    pub clock: Instant,
    pub event_pump: EventPump,
    pub board: Board,
    pub viewport: Viewport,
    pub animation_state: u8,
    pub res: Resources,
}

pub fn setup(sdl: Sdl) -> Result<GameState, failure::Error> {
    let mut res = Resources::from_relative_exe_path("assets").unwrap();
    let board = Board::new(8usize, 6usize, &mut res)?; // this will be the default Board::new(86usize, 64usize, &mut res)?;

    let viewport = Viewport::for_window(900, 700);

    Ok(GameState {
        event_pump: sdl.event_pump().map_err(err_msg)?,
        clock: Instant::now(),
        board,
        viewport,
        animation_state: 0u8,
        res,
    })
}

pub fn run(game: &mut GameState) -> Result<bool, failure::Error> {
    if input::detect_input(game)? {
        return Ok(false);
    }

    game.animation_state = (game.clock.elapsed().as_millis() / 400 % 2) as u8;
    game.board.execute()?;
    game.board.render(&game)?;
    //
    Ok(true)
}
