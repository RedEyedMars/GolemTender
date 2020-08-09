use crate::a::b::tiles::{Occupant, Tile};
use crate::a::e::gol::GolemState;
use crate::a::*;
use packed_simd::u32x2;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Item {
    Key(Key),
    Block,
    ColouredBlock(Colour),
}

impl Item {
    pub fn place(&self, gol: &GolemState, xy: u32x2, board: &mut Board) -> Result<GolemState, ()> {
        let tile_xy = gol.advance(xy, board.width as u32, board.height as u32)?;
        match self {
            Item::Key(k) => board.apply_on_tile(k, tile_xy, &Tile::place_key)?,
            Item::Block => board.apply_on_tile(&Block::Block, tile_xy, &Tile::place_block)?,
            Item::ColouredBlock(c) => board.apply_on_tile(
                &Block::ColouredBlock(c.clone()),
                tile_xy,
                &Tile::place_block,
            )?,
        };
        board
            .apply_on_tile(
                &Occupant::Golem(gol.without_carry()?),
                xy,
                &Tile::replace_occupant,
            )?
            .get_golem()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Key {
    Key,
    ColouredKey(Colour),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Block {
    Block,
    ColouredBlock(Colour),
    StepSwitch { on: Option<Box<Occupant>> },
    PullSwitch,
    FlipSwitch { on: bool },
    ColourSwitch(Colour),
    PullDoor,
    KeyDoor { required: Key, open: bool }, // door activated by a key
    SwitchDoor,                            // door activated by a switch
    KeyPedestal(Option<Key>),
}
impl Block {
    pub fn entity_type(&self) -> EntityType {
        use crate::a::b::Block::*;
        use EntityType::*;
        match self {
            crate::a::b::Block::Block => EntityType::Block,
            ColouredBlock(_) => EntityType::Block,
            StepSwitch { .. } => Switch,
            PullSwitch => Switch,
            ColourSwitch(_) => Switch,
            FlipSwitch { .. } => Switch,
            PullDoor => Door,
            SwitchDoor => Door,
            KeyDoor { .. } => Door,
            KeyPedestal(Some(_)) => EntityType::Block,
            KeyPedestal(None) => Nothing,
        }
    }

    pub fn place_key(&self, key: &Key) -> Result<Block, ()> {
        use self::Block::*;
        match self {
            KeyDoor {
                required: k,
                open: false,
            } if k == key => Ok(KeyDoor {
                required: k.clone(),
                open: true,
            }),
            _ => Err(()),
        }
    }

    pub fn place_block(&self, block: &Block) -> Result<Block, ()> {
        use self::Block::*;
        match self {
            StepSwitch { on: None } => Ok(StepSwitch {
                on: Some(Box::new(Occupant::Block(block.clone()))),
            }),
            _ => Err(()),
        }
    }

    pub fn is_pushable(&self) -> bool {
        use self::Block::*;
        match self {
            crate::a::b::Block::Block => true,
            ColouredBlock(_) => true,
            StepSwitch { .. } => false,
            PullSwitch => false,
            ColourSwitch(_) => false,
            FlipSwitch { .. } => false,
            PullDoor => false,
            SwitchDoor => false,
            KeyDoor { .. } => false,
            KeyPedestal(Some(_)) => false,
            KeyPedestal(None) => false,
        }
    }

    pub fn get_golem(&self) -> Result<GolemState, ()> {
        use self::Block::*;
        match self {
            StepSwitch {
                on: Some(box Occupant::Golem(g)),
            } => Ok(g.clone()),
            _ => Err(()),
        }
    }

    pub fn boarding(&self, xy: u32x2) -> (u32x2, Block) {
        (xy, self.clone())
    }
    pub fn boarding_occ(&self, xy: u32x2) -> (u32x2, Occupant) {
        (xy, Occupant::Block(self.clone()))
    }
}
