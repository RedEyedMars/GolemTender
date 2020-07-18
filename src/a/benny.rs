use crate::a::{Direction, GameState};
use packed_simd::u32x2;

pub fn test_advance_to_edge(game: &mut GameState) {
    use crate::a::e::gol::Golem;
    use crate::a::e::runes::{ActionGlyph, Rune, RuneArrangement};
    game.board.render_text();
    let mut golem = Golem::new_with_direction(packed_simd::u32x2::new(0, 2), Direction::East);
    let mut arrangement = RuneArrangement::new();
    arrangement.add(Rune::basic(ActionGlyph::Advance, 1));
    golem.add_arrangement(arrangement);
    let _ = game.board.add_golem(golem.boarding());
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
}
pub fn test_advance_around_edge(game: &mut GameState) {
    use crate::a::e::gol::Golem;
    use crate::a::e::runes::{ActionGlyph, Rune, RuneArrangement};
    game.board.render_text();
    let mut golem = Golem::new_with_direction(u32x2::new(0, 2), Direction::North);
    let mut arrangement = RuneArrangement::new();
    arrangement.add(Rune::basic(ActionGlyph::Advance, 1));
    arrangement.add(Rune::basic(ActionGlyph::Advance, 1));
    arrangement.add(Rune::basic(ActionGlyph::TurnDex, 1));
    golem.add_arrangement(arrangement);
    let _ = game.board.add_golem(golem.boarding());
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
}
pub fn test_push_to_edge(game: &mut GameState) {
    use crate::a::b::blocks::Block;
    use crate::a::e::gol::Golem;
    use crate::a::e::runes::{ActionGlyph, Rune, RuneArrangement};
    game.board.render_text();
    let mut golem = Golem::new_with_direction(u32x2::new(0, 2), Direction::East);
    let mut arrangement = RuneArrangement::new();
    arrangement.add(Rune::basic(ActionGlyph::Advance, 1));
    golem.add_arrangement(arrangement);
    game.board
        .add_golem(golem.boarding())
        .expect("Could not add golem!");
    game.board
        .add_block(Block::Block.boarding(u32x2::new(1, 2)))
        .expect("Could not add block");
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
}

pub fn test_grab_and_rotate(game: &mut GameState) {
    use crate::a::b::blocks::Block;
    use crate::a::e::gol::Golem;
    use crate::a::e::runes::{ActionGlyph, ArrangementManipGlyph, Rune, RuneArrangement};
    game.board.render_text();
    let mut golem = Golem::new_with_direction(u32x2::new(1, 2), Direction::West);
    let mut arrangement = RuneArrangement::new();
    arrangement.add(Rune::basic(ActionGlyph::Grab, 1));
    arrangement.add(Rune::manip(ArrangementManipGlyph::Next { times: 1 }, 1));
    golem.add_arrangement(arrangement);
    let mut arrangement = RuneArrangement::new();
    arrangement.add(Rune::basic(ActionGlyph::TurnDex, 1));
    arrangement.add(Rune::manip(ArrangementManipGlyph::Repeat, 1));
    golem.add_arrangement(arrangement);
    game.board
        .add_golem(golem.boarding())
        .expect("Could not add golem!");
    game.board
        .add_block(Block::Block.boarding(u32x2::new(0, 2)))
        .expect("Could not add block");
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
}

pub fn test_grab_and_rotate_2(game: &mut GameState) {
    use crate::a::b::blocks::Block;
    use crate::a::e::gol::Golem;
    use crate::a::e::runes::{ActionGlyph, ArrangementManipGlyph, Rune, RuneArrangement};
    game.board.render_text();
    let mut golem = Golem::new_with_direction(u32x2::new(1, 2), Direction::West);
    let mut arrangement = RuneArrangement::new();
    arrangement.add(Rune::basic(ActionGlyph::Grab, 1));
    arrangement.add(Rune::manip(ArrangementManipGlyph::Next { times: 1 }, 1));
    golem.add_arrangement(arrangement);
    let mut arrangement = RuneArrangement::new();
    arrangement.add(Rune::basic(ActionGlyph::TurnDex, 1));
    arrangement.add(Rune::manip(ArrangementManipGlyph::Repeat, 1));
    golem.add_arrangement(arrangement);
    game.board
        .add_golem(golem.boarding())
        .expect("Could not add golem!");
    game.board
        .add_block(Block::Block.boarding(u32x2::new(0, 2)))
        .expect("Could not add block");
    game.board
        .add_block(Block::Block.boarding(u32x2::new(2, 2)))
        .expect("Could not add block");
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
}

pub fn test_lift_and_rotate(game: &mut GameState) {
    use crate::a::b::blocks::Block;
    use crate::a::e::gol::Golem;
    use crate::a::e::runes::{ActionGlyph, Rune, RuneArrangement};
    game.board.render_text();
    let mut golem = Golem::new_with_direction(u32x2::new(1, 2), Direction::West);
    let mut arrangement = RuneArrangement::new();
    arrangement.add(Rune::basic(ActionGlyph::Lift, 1));
    arrangement.add(Rune::basic(ActionGlyph::TurnDex, 2));
    arrangement.add(Rune::basic(ActionGlyph::Place, 1));
    golem.add_arrangement(arrangement);
    game.board
        .add_golem(golem.boarding())
        .expect("Could not add golem!");
    game.board
        .add_block(Block::Block.boarding(u32x2::new(0, 2)))
        .expect("Could not add block");
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
    golem.act(&mut game.board);
    game.board.render_text();
}
