use crate::a::b::blocks::{Block, Key};
use crate::a::b::Board;
use crate::a::b::Wall;
use crate::a::e::gol::Golem;
use crate::a::e::runes::{ActionGlyph, ArrangementManipGlyph, Rune, RuneArrangement};
use crate::a::GameState;
use crate::a::{Colour, Direction};
use crate::s::{tokenize, Token};
use crate::s::{ConditionRuneToken, RuneToken};
use crate::w::g::res::Resources;

use packed_simd::u32x2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fetch_level(filename: &str) -> String {
    String::from_utf8(nodejs_helper::fs::read_file_sync(&filename)).unwrap()
}

pub fn read_level(level_name: &str, game: &mut GameState) -> Result<(), failure::Error> {
    let mut filename = String::from("./res/levels/");
    filename.push_str(level_name);
    filename.push_str(".lvl");
    read_level_from_string(
        //&crate::s::read_file(filename.as_str()).unwrap().into_bytes(),
        fetch_level(filename.as_str()),
        game,
    )
}

#[no_mangle]
pub fn read_level_from_string(
    filecontents: String,
    game: &mut GameState,
) -> Result<(), failure::Error> {
    game.boards = Vec::with_capacity(512);
    game.golems = Vec::with_capacity(8184);
    let mut board_len = 0usize;
    for l in filecontents.lines() {
        board_len = read_level_line(l, board_len, game)?;
    }
    if game.boards.len() > 0 {
        game.board_index = 0usize;
        game.board = game.boards.get(game.board_index).unwrap().clone();
    }
    println!("Complete?");
    Ok(())
}

pub fn read_level_line(
    line: &str,
    size: usize,
    game: &mut GameState,
) -> Result<usize, failure::Error> {
    Ok(match line.chars().nth(0) {
        Some('>') => {
            game.boards.push(read_board(&line[1..], &mut game.res)?);
            if size == 0usize {
                size
            } else {
                size + 1
            }
        }
        Some('G') => {
            read_golem(
                &line[1..],
                &mut game
                    .boards
                    .get_mut(size)
                    .expect("Could not associate golem to any board"),
                &mut game.golems,
                &mut game.res,
            );
            size
        }
        Some('B') => {
            read_block(
                &line[1..],
                &mut game
                    .boards
                    .get_mut(size)
                    .expect("Could not associate golem to any board"),
                &mut game.res,
            )
            .expect("Could not place block on board");
            size
        }
        _ => size,
    })
}

pub fn read_board(line: &str, res: &mut Resources) -> Result<Board, failure::Error> {
    let tokens = tokenize(line);
    println!("{:?}", tokens);
    match tokens.as_slice() {
        [Token::String(name), Token::Coord(width, height)] => {
            Board::new(name.clone(), *width as usize, *height as usize, res)
        }
        _ => Err(failure::err_msg(format!(
            "Unable to parse {} into new Board",
            line
        ))),
    }
}
pub fn read_golem(line: &str, board: &mut Board, golems: &mut Vec<Golem>, res: &mut Resources) {
    let tokens = tokenize(line);
    match tokens.as_slice() {
        [Token::Varying(colour), Token::Varying(direction), Token::Coord(x, y), Token::Runes(runes)] =>
        {
            let mut golem = Golem::new_basic(
                u32x2::new(*x as u32, *y as u32),
                match direction {
                    'N' => Direction::North,
                    'E' => Direction::East,
                    'W' => Direction::West,
                    'S' => Direction::South,
                    _ => Direction::East,
                },
                Colour::from_varying(*colour),
            );
            for r in runes {
                let mut arrangement = RuneArrangement::new();
                for s in r {
                    match s {
                        RuneToken::Action(c, n, ConditionRuneToken::None) => {
                            arrangement.add(Rune::basic(
                                match c {
                                    'a' => ActionGlyph::Advance,
                                    'b' => ActionGlyph::Backtrack,
                                    'd' => ActionGlyph::TurnDex,
                                    's' => ActionGlyph::TurnSin,
                                    'l' => ActionGlyph::Lift,
                                    'p' => ActionGlyph::Place,
                                    'g' => ActionGlyph::Grab,
                                    'r' => ActionGlyph::Release,
                                    _ => ActionGlyph::Advance,
                                },
                                *n,
                            ))
                        }
                        RuneToken::Manip(c, n, ConditionRuneToken::None) => {
                            arrangement.add(Rune::manip(
                                match c {
                                    'n' => ArrangementManipGlyph::Next { times: *n },
                                    'v' => ArrangementManipGlyph::Previous { times: *n },
                                    'k' => ArrangementManipGlyph::Break,
                                    't' => ArrangementManipGlyph::Repeat,
                                    _ => ArrangementManipGlyph::Next { times: 0 },
                                },
                                *n,
                            ))
                        }
                        _ => {}
                    }
                }
                golem.add_arrangement(arrangement);
            }
            board
                .add_occupant(golem.boarding_occ())
                .expect("Could not add golem!");
            golems.push(golem);
        }
        _ => {}
    }
}

pub fn read_block(line: &str, board: &mut Board, res: &mut Resources) -> Result<(), ()> {
    let tokens = tokenize(line);

    /*
    Block/
    ColouredBlock(Colour),/
    StepSwitch { on: Option<Box<Occupant>> },
    PullSwitch,
    FlipSwitch { on: bool },
    ColourSwitch(Colour),
    PullDoor,
    KeyDoor { required: Key, open: bool }, // door activated by a key
    SwitchDoor,                            // door activated by a switch
    KeyPedestal(Option<Key>),
    */
    match tokens.as_slice() {
        [Token::Varying('b'), Token::Coord(x, y)] => {
            board.add_occupant(Block::Block.boarding_occ(u32x2::new(*x as u32, *y as u32)))
        }
        [Token::Varying('b'), Token::Varying(colour), Token::Coord(x, y)] => board.add_occupant(
            Block::ColouredBlock(Colour::from_varying(*colour))
                .boarding_occ(u32x2::new(*x as u32, *y as u32)),
        ),
        [Token::Varying('s'), Token::Coord(x, y)] => board.add_occupant(
            Block::StepSwitch { on: None }.boarding_occ(u32x2::new(*x as u32, *y as u32)),
        ),
        [Token::Varying('p'), Token::Coord(x, y)] => {
            board.add_occupant(Block::PullSwitch.boarding_occ(u32x2::new(*x as u32, *y as u32)))
        }
        [Token::Varying('f'), Token::Coord(x, y)] => board.add_occupant(
            Block::FlipSwitch { on: false }.boarding_occ(u32x2::new(*x as u32, *y as u32)),
        ),
        [Token::Varying('f'), Token::Varying(on), Token::Coord(x, y)] => board.add_occupant(
            Block::FlipSwitch { on: *on == 'O' }.boarding_occ(u32x2::new(*x as u32, *y as u32)),
        ),
        [Token::Varying('s'), Token::Varying(colour), Token::Coord(x, y)] => board.add_occupant(
            Block::ColourSwitch(Colour::from_varying(*colour))
                .boarding_occ(u32x2::new(*x as u32, *y as u32)),
        ),
        [Token::Varying('D'), Token::Coord(x, y)] => {
            board.add_occupant(Block::PullDoor.boarding_occ(u32x2::new(*x as u32, *y as u32)))
        }
        [Token::Varying('D'), Token::Varying('K'), Token::Coord(x, y)] => board.add_occupant(
            Block::KeyDoor {
                required: Key::Key,
                open: false,
            }
            .boarding_occ(u32x2::new(*x as u32, *y as u32)),
        ),
        [Token::Varying('D'), Token::Varying('K'), Token::Varying(open), Token::Coord(x, y)] => {
            board.add_occupant(
                Block::KeyDoor {
                    required: Key::Key,
                    open: *open == 'O',
                }
                .boarding_occ(u32x2::new(*x as u32, *y as u32)),
            )
        }
        [Token::Varying('D'), Token::Varying('K'), Token::Varying(open), Token::Varying(colour), Token::Coord(x, y)] => {
            board.add_occupant(
                Block::KeyDoor {
                    required: Key::ColouredKey(Colour::from_varying(*colour)),
                    open: *open == 'O',
                }
                .boarding_occ(u32x2::new(*x as u32, *y as u32)),
            )
        }

        [Token::Varying('D'), Token::Varying('s'), Token::Coord(x, y)] => {
            board.add_occupant(Block::SwitchDoor.boarding_occ(u32x2::new(*x as u32, *y as u32)))
        }
        [Token::Varying('I'), Token::Varying('K'), Token::Varying('K'), Token::Coord(x, y)] => {
            board.add_occupant(
                Block::KeyPedestal(Some(Key::Key)).boarding_occ(u32x2::new(*x as u32, *y as u32)),
            )
        }
        [Token::Varying('I'), Token::Varying('K'), Token::Varying(colour), Token::Coord(x, y)] => {
            board.add_occupant(
                Block::KeyPedestal(Some(Key::ColouredKey(Colour::from_varying(*colour))))
                    .boarding_occ(u32x2::new(*x as u32, *y as u32)),
            )
        }
        [Token::Varying('I'), Token::Varying('K'), Token::Coord(x, y)] => board
            .add_occupant(Block::KeyPedestal(None).boarding_occ(u32x2::new(*x as u32, *y as u32))),
        _ => Ok(()),
    }
}
pub fn save_level(level_name: &str, game: &GameState) -> Result<(), failure::Error> {
    let mut tokens = Vec::with_capacity(2048);
    save_board_line(&game.board, &mut tokens);
    for g in game.golems.iter() {
        save_golem_line(&g, &mut tokens);
    }
    let mut filename = String::from("./res/levels/");
    filename.push_str(level_name);
    filename.push_str(".lvl");
    crate::s::write_file(filename.as_str(), &tokens)
}
fn save_board_line(board: &Board, tokens: &mut Vec<Vec<Token>>) {
    tokens.push(vec![
        Token::Varying('>'),
        Token::String(board.name.clone()),
        Token::Coord(board.width as i32, board.height as i32),
    ]);
    for (wall, xy) in board.walls().iter() {
        save_wall_line(wall, *xy, tokens);
    }
    for (block, xy) in board.blocks().iter() {
        save_block_line(block, *xy, tokens);
    }
}

fn save_golem_line(golem: &Golem, tokens: &mut Vec<Vec<Token>>) {
    let mut arrangement = Vec::new();
    for rs in golem.get_arrangement().iter() {
        let mut runes = Vec::new();
        for r in rs.get_runes().iter() {
            if let Some(action) = r.get_action() {
                runes.push(RuneToken::Action(
                    match action {
                        ActionGlyph::Advance => 'a',
                        ActionGlyph::Backtrack => 'b',
                        ActionGlyph::TurnDex => 'd',
                        ActionGlyph::TurnSin => 's',
                        ActionGlyph::Lift => 'l',
                        ActionGlyph::Place => 'p',
                        ActionGlyph::Grab => 'g',
                        ActionGlyph::Release => 'r',
                    },
                    r.get_action_times(),
                    ConditionRuneToken::None,
                ));
            } else if let Some(manip) = r.get_manip() {
                runes.push(RuneToken::Manip(
                    match manip {
                        ArrangementManipGlyph::Next { .. } => 'n',
                        ArrangementManipGlyph::Previous { .. } => 'v',
                        ArrangementManipGlyph::Break => 'k',
                        ArrangementManipGlyph::Repeat => 't',
                    },
                    r.get_manip_times(),
                    ConditionRuneToken::None,
                ));
            }
        }
        arrangement.push(runes);
    }
    let pos = golem.get_position();
    tokens.push(vec![
        Token::Varying('G'),
        Token::Varying(golem.get_colour().to_varying()),
        match golem.get_direction() {
            Direction::North => Token::Varying('N'),
            Direction::East => Token::Varying('E'),
            Direction::West => Token::Varying('W'),
            Direction::South => Token::Varying('S'),
        },
        Token::Coord(pos.extract(0) as i32, pos.extract(1) as i32),
        Token::Runes(arrangement),
    ]);
}

pub fn save_block_line(block: &Block, xy: u32x2, tokens: &mut Vec<Vec<Token>>) {
    use crate::a::b::tiles::Occupant;
    match block {
        Block::Block => {
            tokens.push(vec![
                Token::Varying('B'),
                Token::Varying('b'),
                Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
            ]);
        }
        Block::ColouredBlock(colour) => {
            tokens.push(vec![
                Token::Varying('B'),
                Token::Varying('b'),
                Token::Varying(colour.to_varying()),
                Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
            ]);
        }
        Block::StepSwitch { on: on } => {
            tokens.push(vec![
                Token::Varying('B'),
                Token::Varying('s'),
                Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
            ]);
            if let Some(box occupant) = on {
                match occupant {
                    Occupant::Block(b) => save_block_line(b, xy, tokens),
                    Occupant::Wall(w) => save_wall_line(w, xy, tokens),
                    Occupant::Golem(_) => {
                        // These will be saved in the save golems step
                    }
                }
            }
        }
        Block::PullSwitch => {
            tokens.push(vec![
                Token::Varying('B'),
                Token::Varying('p'),
                Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
            ]);
        }
        Block::FlipSwitch { on: true } => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('f'),
            Token::Varying('O'),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::FlipSwitch { on: false } => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('f'),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::ColourSwitch(colour) => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('s'),
            Token::Varying(colour.to_varying()),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::PullDoor => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('D'),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::SwitchDoor => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('D'),
            Token::Varying('s'),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::KeyDoor {
            open: true,
            required: Key::Key,
        } => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('D'),
            Token::Varying('K'),
            Token::Varying('O'),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::KeyDoor {
            open: false,
            required: Key::Key,
        } => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('D'),
            Token::Varying('K'),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::KeyDoor {
            open: true,
            required: Key::ColouredKey(colour),
        } => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('D'),
            Token::Varying('K'),
            Token::Varying('O'),
            Token::Varying(colour.to_varying()),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::KeyDoor {
            open: false,
            required: Key::ColouredKey(colour),
        } => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('D'),
            Token::Varying('K'),
            Token::Varying(colour.to_varying()),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::KeyPedestal(None) => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('I'),
            Token::Varying('K'),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::KeyPedestal(Some(Key::Key)) => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('I'),
            Token::Varying('K'),
            Token::Varying('K'),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
        Block::KeyPedestal(Some(Key::ColouredKey(colour))) => tokens.push(vec![
            Token::Varying('B'),
            Token::Varying('I'),
            Token::Varying('K'),
            Token::Varying(colour.to_varying()),
            Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
        ]),
    }
}

pub fn save_wall_line(_wall: &Wall, xy: u32x2, tokens: &mut Vec<Vec<Token>>) {
    tokens.push(vec![
        Token::Varying('W'),
        Token::Coord(xy.extract(0) as i32, xy.extract(1) as i32),
    ]);
}
