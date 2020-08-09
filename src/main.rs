#![feature(stmt_expr_attributes)]
#![feature(box_syntax, box_patterns)]
extern crate gl;
extern crate log;
extern crate sdl2;
extern crate vec_2_10_10_10;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate soa_derive;
//#[macro_use]
//extern crate auto_claw_render_gl_derive as render_gl_derive;

extern crate generational_arena;
extern crate maplit;
extern crate packed_simd;
extern crate rand;
extern crate rayon;

pub mod a;
pub mod g;
pub mod n;
pub mod s;

use log::LevelFilter;
use log::{Level, Metadata, Record};

struct SimpleLogger;
static LOGGER: SimpleLogger = SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
pub fn main() -> Result<(), failure::Error> {
    match log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info)) {
        Err(err) => println!("{:?}", err),
        _ => {}
    }
    let (sdl, mut window, _gl_context) = g::setup()?;
    let mut game = a::setup(sdl)?;
    //let mut net = n::s::node::Net::new(1024, 64, vec![4096, 4096, 2048]);
    //a::benny::test_nn_32x32_push_performance(&mut net);

    a::benny::test_read_write_file(&mut game);
    loop {
        g::preprocess();
        if !a::run(&mut game)? {
            break;
        }
        g::postprocess(&mut window);
    }

    Ok(())
}
