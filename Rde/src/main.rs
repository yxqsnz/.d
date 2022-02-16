#![allow(non_snake_case)]
mod apps;
mod wm;
mod utils;
mod consts;
use wm::window::WindowManager;

fn main() {
    let display = cursive::default();
    let mut manager = WindowManager::new(display);

    manager.init();
}
