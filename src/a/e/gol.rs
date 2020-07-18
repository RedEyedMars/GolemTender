//use generational_arena::Index;
use packed_simd::u32x2;

use crate::a::b::blocks::Item;
use crate::a::b::Board;
use crate::a::e::runes::RuneArrangement;
use crate::a::{Colour, Direction};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum GolemState {
    State {
        direction: Direction,
        colour: Colour,
        carrying: Option<Item>,
        grabbing: bool,
    },
}

impl GolemState {
    pub fn from_direction(&self, t: &dyn Fn(Direction) -> Direction) -> GolemState {
        match self.clone() {
            GolemState::State {
                direction: d,
                colour: c,
                carrying: car,
                grabbing: g,
            } => GolemState::State {
                direction: t(d),
                colour: c,
                carrying: car,
                grabbing: g,
            },
        }
    }

    pub fn facing(&self) -> Direction {
        match self {
            GolemState::State { direction: d, .. } => d.clone(),
        }
    }

    pub fn advance(&self, xy: u32x2, width: u32, height: u32) -> Result<u32x2, ()> {
        match self {
            GolemState::State { direction: d, .. } => Ok(d.advance(xy, width, height)?), // _ => Err(()),
        }
    }
    pub fn retreat(&self, xy: u32x2, width: u32, height: u32) -> Result<u32x2, ()> {
        match self {
            GolemState::State { direction: d, .. } => Ok(d.retreat(xy, width, height)?), // _ => Err(()),
        }
    }
    pub fn advance_and_rotated(
        &self,
        xy: u32x2,
        width: u32,
        height: u32,
        rotation_function: &dyn Fn(Direction) -> Direction,
    ) -> Result<(u32x2, u32x2), ()> {
        match self {
            GolemState::State { direction: d, .. } => Ok((
                d.advance(xy, width, height)?,
                rotation_function(d.clone()).advance(xy, width, height)?,
            )), // _ => Err(()),
        }
    }

    pub fn grabbing(&self) -> bool {
        match self {
            GolemState::State { grabbing: g, .. } => g.clone(),
        }
    }

    pub fn place(&self, xy: u32x2, board: &mut Board) -> Result<GolemState, ()> {
        if let Some(item) = match self {
            GolemState::State { carrying, .. } => carrying,
        } {
            Ok(item.place(self, xy, board)?)
        } else {
            Err(())
        }
    }

    pub fn without_carry(&self) -> Result<GolemState, ()> {
        match self {
            GolemState::State {
                carrying: Some(_),
                direction: d,
                grabbing: g,
                colour: c,
            } => Ok(GolemState::State {
                carrying: None,
                direction: d.clone(),
                grabbing: g.clone(),
                colour: c.clone(),
            }),
            _ => Err(()),
        }
    }
}

pub struct Golem {
    selected_arrangement: Vec<usize>,
    runes: Vec<RuneArrangement>,
    pub state: GolemState,
    pub position: u32x2,
}

impl Golem {
    pub fn new_with_direction(xy: u32x2, direction: Direction) -> Golem {
        Golem {
            position: xy,
            state: GolemState::State {
                direction: direction,
                colour: Colour::Blue,
                carrying: None,
                grabbing: false,
            },
            runes: Vec::new(),
            selected_arrangement: Vec::new(),
        }
    }
    pub fn add_arrangement(&mut self, ra: RuneArrangement) {
        self.runes.push(ra);
        if self.selected_arrangement.len() == 0 {
            self.selected_arrangement.push(0);
        }
    }
    pub fn boarding(&self) -> (u32x2, GolemState) {
        (self.position.clone(), self.state.clone())
    }
    pub fn selected(&self) -> usize {
        self.selected_arrangement
            .get(self.selected_arrangement.len() - 1)
            .unwrap()
            .clone()
    }
    pub fn act(&mut self, board: &mut Board) {
        let selected = self.selected();
        self.runes.get(selected).unwrap().clone().act(self, board);
        println!("{:?}", self.state);
        if self.runes.get_mut(selected).unwrap().inc()
            && selected == self.selected()
            && self.selected_arrangement.len() > 1
        {
            // if arrangement is now selecting 0th rune, then we should pop it off the stack
            self.selected_arrangement.pop();
        }
    }
    pub fn next(&mut self, _board: &mut Board, times: usize) -> Result<(), ()> {
        let selected = self.selected();
        if selected >= self.runes.len() - times {
            self.selected_arrangement
                .push(self.runes.len() - (selected + times + 1));
        } else {
            self.selected_arrangement.push(selected + times);
        }
        Ok(())
    }
    pub fn previous(&mut self, _board: &mut Board, times: usize) -> Result<(), ()> {
        let selected = self.selected();
        if selected < times {
            self.selected_arrangement
                .push(self.runes.len() - (times - selected));
        } else {
            self.selected_arrangement.push(selected - times);
        }
        Ok(())
    }
    pub fn repeat(&mut self, _board: &mut Board) -> Result<(), ()> {
        let selected = self.selected();
        self.runes.get_mut(selected).unwrap().repeat();
        Ok(())
    }
    pub fn breakout(&mut self, _board: &mut Board) -> Result<(), ()> {
        let selected = self.selected();
        self.runes.get_mut(selected).unwrap().breakout();
        Ok(())
    }
    pub fn advance(&mut self, board: &mut Board) -> Result<(), ()> {
        self.position = board.advance(self.position)?;
        Ok(())
    }
    pub fn retreat(&mut self, board: &mut Board) -> Result<(), ()> {
        let (new_position, new_state) = board.retreat(self.position)?;
        self.position = new_position;
        self.state = new_state;
        Ok(())
    }
    pub fn turn_dex(&mut self, board: &mut Board) -> Result<(), ()> {
        self.state = board.turn(self.position, &Direction::turn_dex)?;
        Ok(())
    }
    pub fn turn_sin(&mut self, board: &mut Board) -> Result<(), ()> {
        self.state = board.turn(self.position, &Direction::turn_sin)?;
        Ok(())
    }
    pub fn lift(&mut self, board: &mut Board) -> Result<(), ()> {
        self.state = board.lift(self.position)?;
        Ok(())
    }
    pub fn place(&mut self, board: &mut Board) -> Result<(), ()> {
        self.state = self.state.place(self.position, board)?;
        Ok(())
    }
    pub fn grab(&mut self, board: &mut Board) -> Result<(), ()> {
        self.state = board.grab(true, self.position)?;
        Ok(())
    }
    pub fn release(&mut self, board: &mut Board) -> Result<(), ()> {
        self.state = board.grab(false, self.position)?;
        Ok(())
    }
}
