mod engine;
mod gui;

use engine::GVPengine;

fn main() {
    let engine = GVPengine::init();

    'main_loop: loop {
        if engine.poll_events() {
            break 'main_loop;
        }
    }
}
