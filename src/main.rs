mod engine;
mod gui;

use engine::GVPengine;

fn main() {
  let engine = GVPengine::init();
  engine.run();
}