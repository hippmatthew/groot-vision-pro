mod gvp_engine;

use gvp_engine::engine::GVPEngine;

fn main() {
  let engine = GVPEngine::init();

  'main_loop: loop {
    if engine.poll_events() { break 'main_loop; };
  }
}
