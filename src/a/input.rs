//use crate::a::b::Board;
use crate::a::GameState;

pub fn detect_input(game: &mut GameState) -> Result<bool, failure::Error> {
    for event in game.event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit { .. } => return Ok(true),
            sdl2::event::Event::Window {
                win_event: sdl2::event::WindowEvent::Resized(w, h),
                ..
            } => {
                game.viewport.update_size(w, h);
                game.viewport.set_used();
            }
            sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                game.board.md(
                    (x as f32 - game.viewport.w as f32 / 2f32) / game.viewport.w as f32,
                    (y as f32 - game.viewport.h as f32 / 2f32) / game.viewport.h as f32,
                )?;
            }
            sdl2::event::Event::MouseButtonUp { .. } => {
                game.board.mu()?;
            }
            _ => {}
        }
    }
    Ok(false)
}
