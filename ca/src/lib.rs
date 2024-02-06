//! # Cellular Automaton evaluation Program
//!
//! Set up the automaton struct with a grid, seed, and ruleset (Game of Life, 
//! Brian's Brain, etc.) and run to view in the terminal or write result to 
//! a file to use elsewhere. 
//!
//! ##  Todo:
//! - [ ] Points 
//!     - [x] Implemention
//!     - [ ] Testing
//!     - [ ] Documentation
//! - [ ] Grid
//!     - [x] Implemention
//!     - [ ] Testing
//!     - [ ] Documentation
//! - [ ] Automaton
//!     - [ ] Implemention
//!     - [ ] Testing
//!     - [ ] Documentation
//! - [ ] Rules 
//!     - [x] Implemention
//!     - [ ] Testing
//!     - [ ] Documentation

pub mod automaton;
mod errors;
pub mod grid;
pub mod parse;
pub mod rule;
pub mod state;

