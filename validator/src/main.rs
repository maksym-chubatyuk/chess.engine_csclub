// Very annoying for a work in progress
#![allow(dead_code)]
pub mod game;
use game::*;

fn main() {
    let state = GameState::new();
    println!("{state}");
}
