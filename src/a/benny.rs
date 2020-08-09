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

pub fn test_nn_32x32_push_performance(net: &mut crate::n::s::node::Net) {
    let (low, high) = (-1f32, 1f32);
    let mut rng = rand::thread_rng();

    let mut input = Vec::new();
    for _ in 0..1024 / 16 {
        input.push(crate::n::s::node::Cluster::random_weights((
            &mut rng, low, high,
        )));
    }
    net.input(input);
    //net.display_full();
    //println!("=========");
    use std::time::Instant;
    let clock = Instant::now();
    let mut times = Vec::new();
    println!("Start test");
    for _ in 0..20 {
        let start = clock.elapsed().as_nanos();
        net.push();
        net.push();
        net.push();
        net.push();
        net.push();
        net.push();
        net.push();
        net.push();
        net.push();
        net.push();
        let end = clock.elapsed().as_nanos();
        times.push((end - start) / 10);
    }
    let mut average = 0;
    for t in times.iter() {
        average += t;
    }
    println!("Push takes {}ns", average / (times.len() as u128));
    //net.display_full();
}

pub fn test_nn_1x1_push_pull(net: &mut crate::n::s::node::Net) {
    use rand::Rng;
    let (low, high) = (-1f32, 1f32);
    let mut rng = rand::thread_rng();
    //net.display_full();
    //println!("=========");
    use packed_simd::f32x16;
    use std::time::Instant;
    let clock = Instant::now();
    let mut times = Vec::new();
    for _ in 0..1000 {
        let start = clock.elapsed().as_millis();
        let (a, b) = (rng.gen_range(low, high), rng.gen_range(low, high));
        if a == 0.0 || b == 0.0 {
            continue;
        }
        net.input(vec![f32x16::new(
            a, b, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        )]);
        net.push();
        net.pull(vec![f32x16::new(
            a * b,
            a / b,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        )]);
        let end = clock.elapsed().as_millis();
        times.push((end - start));
    }
    let mut average = 0;
    for t in times.iter() {
        average += t;
    }
    println!("Push Pull takes {}ms", average / (times.len() as u128));
    //net.display_full();
}

pub fn test_read_file(game: &mut crate::a::GameState) {
    use crate::s::l::read_level;
    read_level("test_a", game).expect("Found error");
}

pub fn test_read_write_file(game: &mut crate::a::GameState) {
    use crate::s::l::{read_level, save_level};
    read_level("test_b", game).expect("Found error");
    save_level("test_c", game).expect("Found error");
}
