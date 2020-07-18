use crate::a::b::blocks::{Block, Item, Key};
use crate::a::b::Wall;
use crate::a::e::gol::GolemState;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Occupant {
    Golem(GolemState),
    Block(Block),
    Wall(Wall),
}

impl Occupant {
    pub fn render_text(&self) {
        match self {
            Occupant::Block(_) => print!("b|"),
            Occupant::Golem(_) => print!("G|"),
            Occupant::Wall(_) => print!("W|"),
        }
    }
    pub fn pushable_occupant(&self) -> Option<Occupant> {
        match self {
            Occupant::Block(Block::Block) => Some(Occupant::Block(Block::Block)),
            Occupant::Block(Block::ColouredBlock(c)) => {
                Some(Occupant::Block(Block::ColouredBlock(c.clone())))
            }
            Occupant::Block(Block::StepSwitch {
                on: Some(box occupant),
            }) => Some(occupant.clone()),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Tile {
    Empty,
    Occupied(Occupant),
}
impl Tile {
    pub fn render_text(&self) {
        use Tile::*;
        match self {
            Empty => print!("_|"),
            Occupied(o) => o.render_text(),
        }
    }
    pub fn grab(&self, new_state: &bool) -> Result<Tile, ()> {
        use Tile::*;
        match self {
            Occupied(Occupant::Golem(GolemState::State {
                direction: d,
                colour: c,
                grabbing: b,
                carrying: car,
            })) if *b != *new_state => Ok(Tile::Occupied(Occupant::Golem(GolemState::State {
                direction: d.clone(),
                colour: c.clone(),
                grabbing: *new_state,
                carrying: car.clone(),
            }))),
            Empty | Occupied(_) => Err(()),
        }
    }
    pub fn place_key(&self, key: &Key) -> Result<Tile, ()> {
        use Tile::*;
        match self {
            // Place where there is nothing
            Empty => Ok(Tile::Occupied(Occupant::Block(Block::KeyPedestal(Some(
                key.clone(),
            ))))),
            // Give to another golem
            Occupied(Occupant::Golem(GolemState::State {
                direction: d,
                colour: c,
                grabbing: b,
                carrying: None,
            })) => Ok(Tile::Occupied(Occupant::Golem(GolemState::State {
                direction: d.clone(),
                colour: c.clone(),
                grabbing: b.clone(),
                carrying: Some(Item::Key(key.clone())),
            }))),
            // Put in lock?
            Occupied(Occupant::Block(b)) => Ok(Tile::Occupied(Occupant::Block(b.place_key(key)?))),
            // Do nothing
            _ => Err(()),
        }
    }

    pub fn place_block(&self, block: &Block) -> Result<Tile, ()> {
        use Tile::*;
        match self {
            // Place where there is nothing
            Empty => Ok(Tile::Occupied(Occupant::Block(block.clone()))),
            // Give to another golem
            Occupied(Occupant::Golem(GolemState::State {
                direction: d,
                colour: c,
                grabbing: b,
                carrying: None,
            })) => Ok(Tile::Occupied(Occupant::Golem(GolemState::State {
                direction: d.clone(),
                colour: c.clone(),
                grabbing: b.clone(),
                carrying: Some(match block {
                    Block::Block => Ok(Item::Block),
                    Block::ColouredBlock(c) => Ok(Item::ColouredBlock(c.clone())),
                    _ => Err(()),
                }?),
            }))),
            // Put in lock?
            Occupied(Occupant::Block(b)) => {
                Ok(Tile::Occupied(Occupant::Block(b.place_block(block)?)))
            }
            // Do nothing
            _ => Err(()),
        }
    }

    pub fn accept(&self, b: Block) -> Result<Tile, ()> {
        match self {
            Tile::Empty => Ok(Tile::Occupied(Occupant::Block(b))),
            Tile::Occupied(Occupant::Block(Block::StepSwitch { on: None })) => {
                Ok(Tile::Occupied(Occupant::Block(Block::StepSwitch {
                    on: Some(Box::new(Occupant::Block(b))),
                })))
            }
            _ => Err(()),
        }
    }
    pub fn get_golem(&self) -> Result<GolemState, ()> {
        match self {
            Tile::Occupied(Occupant::Golem(g)) => Ok(g.clone()),
            Tile::Occupied(Occupant::Block(b)) => Ok(b.get_golem()?),
            _ => Err(()),
        }
    }
    pub fn add_golem(&self, gol: &GolemState) -> Result<Tile, ()> {
        match self {
            Tile::Empty => Ok(Tile::Occupied(Occupant::Golem(gol.clone()))),
            _ => Err(()),
        }
    }
    pub fn add_block(&self, block: &Block) -> Result<Tile, ()> {
        match self {
            Tile::Empty => Ok(Tile::Occupied(Occupant::Block(block.clone()))),
            _ => Err(()),
        }
    }

    pub fn lift_onto_golem(&self, block: &Block) -> Result<Tile, ()> {
        let carrying = match block {
            Block::Block => Ok(Item::Block),
            Block::ColouredBlock(c) => Ok(Item::ColouredBlock(c.clone())),
            Block::KeyPedestal(Some(k)) => Ok(Item::Key(k.clone())),
            Block::StepSwitch {
                on: Some(box Occupant::Block(Block::Block)),
            } => Ok(Item::Block),
            Block::StepSwitch {
                on: Some(box Occupant::Block(Block::ColouredBlock(c))),
            } => Ok(Item::ColouredBlock(c.clone())),
            _ => Err(()),
        }?;
        match self {
            Tile::Occupied(Occupant::Golem(GolemState::State {
                direction: d,
                colour: c,
                grabbing: b,
                carrying: None,
            })) => Ok(Tile::Occupied(Occupant::Golem(GolemState::State {
                direction: d.clone(),
                colour: c.clone(),
                grabbing: b.clone(),
                carrying: Some(carrying),
            }))),
            _ => Err(()),
        }
    }

    pub fn lift_target(&self, block: &Block) -> Result<Tile, ()> {
        match block {
            Block::Block => Ok(Tile::Empty),
            Block::ColouredBlock(_) => Ok(Tile::Empty),
            Block::KeyPedestal(Some(_)) => Ok(Tile::Empty),
            Block::StepSwitch {
                on: Some(box Occupant::Block(Block::Block)),
            } => Ok(Tile::Occupied(Occupant::Block(Block::StepSwitch {
                on: None,
            }))),
            Block::StepSwitch {
                on: Some(box Occupant::Block(Block::ColouredBlock(_))),
            } => Ok(Tile::Occupied(Occupant::Block(Block::StepSwitch {
                on: None,
            }))),
            Block::FlipSwitch { on: b } => Ok(Tile::Occupied(Occupant::Block(Block::FlipSwitch {
                on: !b,
            }))),
            _ => Err(()),
        }
    }

    pub fn pushable_occupant(&self) -> Option<Occupant> {
        match self {
            Tile::Occupied(o) => o.pushable_occupant(),
            _ => None,
        }
    }
    pub fn is_steppable(&self) -> bool {
        match self {
            Tile::Empty
            | Tile::Occupied(Occupant::Block(Block::Block))
            | Tile::Occupied(Occupant::Block(Block::ColouredBlock(_)))
            | Tile::Occupied(Occupant::Block(Block::StepSwitch { on: None }))
            | Tile::Occupied(Occupant::Block(Block::StepSwitch {
                on: Some(box Occupant::Block(Block::Block)),
            }))
            | Tile::Occupied(Occupant::Block(Block::StepSwitch {
                on: Some(box Occupant::Block(Block::ColouredBlock(_))),
            })) => true,
            _ => false,
        }
    }
    pub fn is_acceptable(&self) -> bool {
        match self {
            Tile::Empty | Tile::Occupied(Occupant::Block(Block::StepSwitch { on: None })) => true,
            _ => false,
        }
    }
    pub fn remove_occupant(&self) -> Result<Tile, ()> {
        match self {
            Tile::Occupied(Occupant::Block(Block::StepSwitch { on: Some(_) })) => {
                Ok(Tile::Occupied(Occupant::Block(Block::StepSwitch {
                    on: None,
                })))
            }
            Tile::Occupied(_) => Ok(Tile::Empty),
            Tile::Empty => Ok(Tile::Empty),
            //_ => Err(()),
        }
    }
    pub fn add_occupant(&self, occupant: Occupant) -> Result<Tile, ()> {
        match self {
            Tile::Occupied(Occupant::Block(Block::StepSwitch { on: None })) => {
                Ok(Tile::Occupied(Occupant::Block(Block::StepSwitch {
                    on: Some(Box::new(occupant)),
                })))
            }
            Tile::Empty => Ok(Tile::Occupied(occupant)),
            _ => Err(()),
        }
    }
    pub fn replace_occupant(&self, occupant: &Occupant) -> Result<Tile, ()> {
        match occupant {
            Occupant::Block(b) => match self {
                Tile::Occupied(Occupant::Block(_)) => {
                    Ok(Tile::Occupied(Occupant::Block(b.clone())))
                }
                _ => Err(()),
            },
            Occupant::Golem(g) => match self {
                Tile::Occupied(Occupant::Golem(_)) => {
                    Ok(Tile::Occupied(Occupant::Golem(g.clone())))
                }
                _ => Err(()),
            },
            Occupant::Wall(w) => match self {
                Tile::Occupied(Occupant::Wall(_)) => Ok(Tile::Occupied(Occupant::Wall(w.clone()))),
                _ => Err(()),
            },
        }
    }
}
