#![allow(unused)]
use pyo3::prelude::*;

mod game;
pub mod prelude;

#[pymodule]
mod validator {

    use super::*;
    use game::GameState;

    #[pyclass]
    struct State {
        internal_state: game::GameState,
    }

    #[pymethods]
    impl State {
        #[new]
        fn new() -> Self {
            State {
                internal_state: GameState::new(),
            }
        }

        /// Returns the a list of all legal moves in
        /// UCI notation
        fn get_legal_moves(&self) -> Vec<String> {
            self.internal_state.get_all_moves()
        }

        /// Attempts to make a move. Returns whether the move was successful.
        fn move_piece(&mut self, movement: String) -> bool {
            self.internal_state.move_piece(movement).is_ok()
        }

        fn print(&self) {
            println!("{}", self.internal_state);
        }
    }
}
