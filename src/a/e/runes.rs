use crate::a::b::Board;
use crate::a::e::gol::Golem;
use crate::a::{Colour, EntityType};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ConditionTarget {
    On,       // .-
    Next,     // -.
    OnOrNext, // .-.
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TileState {
    Colour(Colour),
    Entity(EntityType),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ActionGlyph {
    // Actions
    Advance,   // -()-
    Backtrack, // (|)
    TurnDex,   // -<>-
    TurnSin,   // <|>
    Lift,      // [|]
    Place,     // -[]-
    Grab,      // -#-
    Release,   // #|
}

impl ActionGlyph {
    pub fn get_fn(&self) -> &'static dyn Fn(&mut Golem, &mut Board) -> Result<(), ()> {
        use ActionGlyph::*;
        match self {
            Advance => (&Golem::advance),
            Backtrack => (&Golem::retreat),
            TurnDex => (&Golem::turn_dex),
            TurnSin => (&Golem::turn_sin),
            Lift => (&Golem::lift),
            Place => (&Golem::place),
            Grab => (&Golem::grab),
            Release => (&Golem::release),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ConditionGlyph {
    target: ConditionTarget,
    tile: TileState,
    times: u8,
    not: bool,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ArrangementManipGlyph {
    Next { times: u8 },     // -S-
    Previous { times: u8 }, // -Z-
    Repeat,                 // Spiral
    Break,                  // anti-clockwise spiral
}

impl ArrangementManipGlyph {
    pub fn get_fn(&self) -> Box<dyn Fn(&mut Golem, &mut Board) -> Result<(), ()>> {
        use ArrangementManipGlyph::*;
        match self {
            Next { times } => {
                let t = times.clone();
                Box::new(move |gol: &mut Golem, board: &mut Board| gol.next(board, t as usize))
            }
            Previous { times } => {
                let t = times.clone();
                Box::new(move |gol: &mut Golem, board: &mut Board| gol.previous(board, t as usize))
            }
            Repeat => Box::new(move |gol: &mut Golem, board: &mut Board| gol.repeat(board)),
            Break => Box::new(move |gol: &mut Golem, board: &mut Board| gol.breakout(board)),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Rune {
    action: Option<ActionGlyph>,
    action_times: u8,               // number of lines 0 = 1, and max 3
    condition: Vec<ConditionGlyph>, // all conditions are anded together
    manip: Option<ArrangementManipGlyph>,
}

impl Rune {
    pub fn basic(action: ActionGlyph, times: u8) -> Rune {
        Rune {
            action: Some(action),
            action_times: times,
            condition: Vec::new(),
            manip: None,
        }
    }
    pub fn manip(manip: ArrangementManipGlyph, times: u8) -> Rune {
        Rune {
            action: None,
            action_times: times,
            condition: Vec::new(),
            manip: Some(manip),
        }
    }
    pub fn act(&self, selected_rune: usize, gol: &mut Golem, board: &mut Board) {
        for _ in 0..self.action_times {
            self.act_once(selected_rune, gol, board);
        }
        if let Some(manip) = &self.manip {
            let _ = manip.get_fn()(gol, board);
        }
    }

    pub fn act_once(&self, _selected_rune: usize, gol: &mut Golem, board: &mut Board) {
        if let Some(action) = &self.action {
            let _ = action.get_fn()(gol, board);
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RuneArrangement {
    id: usize,
    selected_rune: usize,
    runes: Vec<Rune>,
    caller: Option<usize>,
    on_repeat: bool,
}

impl RuneArrangement {
    pub fn new() -> RuneArrangement {
        RuneArrangement {
            id: 0,
            selected_rune: 0,
            runes: Vec::new(),
            caller: None,
            on_repeat: false,
        }
    }
    pub fn add(&mut self, rune: Rune) {
        self.runes.push(rune);
    }

    pub fn act(&self, golem: &mut Golem, board: &mut Board) {
        self.runes
            .get(self.selected_rune)
            .unwrap()
            .act(self.selected_rune, golem, board);
    }
    pub fn inc(&mut self) -> bool {
        if self.on_repeat {
            self.on_repeat = false;
            return false;
        }
        // true == rolled over, false == can inc more
        if self.runes.len() == 1 {
            return true;
        } else if self.selected_rune == self.runes.len() - 1 {
            self.selected_rune = 0;
            return true;
        } else {
            self.selected_rune += 1;
            return false;
        }
    }
    pub fn repeat(&mut self) {
        self.selected_rune = 0;
        self.on_repeat = true;
    }
    pub fn breakout(&mut self) {
        self.selected_rune = self.runes.len() - 1;
        self.on_repeat = false;
    }
}
