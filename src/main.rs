#![feature(stmt_expr_attributes)]
#![feature(box_syntax, box_patterns)]
#![feature(start)]
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

pub mod a;
pub mod n;
pub mod s;
pub mod w;

#[start]
pub fn main(argc: isize, argv: *const *const u8) -> isize {
    let mut context = w::g::setup();
    let mut game = a::setup(context).expect("game did not setup properly");
    //let mut net = n::s::node::Net::new(1024, 64, vec![4096, 4096, 2048]);
    //a::benny::test_nn_32x32_push_performance(&mut net);

    a::benny::test_read_file(&mut game);
    //loop {
    //g::preprocess();
    if !a::run(&mut game).expect("game did not run properly") {
        //break;
    }
    //g::postprocess(&mut window);
    //}
    0
}
