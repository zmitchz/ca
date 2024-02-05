use std::ops::{Add, Mul};

use crate::grid::apoint::APoint;
use crate::grid::grid_2d::Grid2D;
use crate::grid::point::IsPoint;
use crate::rule::{Rule, Rules};
use crate::state::State;

/// struct containing all the cells, rules / transitions, and cells that need 
/// to be updated for the current step 
pub struct Automaton2D<'a, T, U> 
    where 
    U: Fn(State<T>) -> State<T>, {
    grid: Grid2D<'a, APoint<'a, i32, State<'a, T>>, i32>, 
    rule: Rules<State<'a, T>, U>,
    update_cells:  Option<Vec<&'a T>>
}

impl<'a, T, U> Automaton2D<'a, T, U>
where
    U: Fn(State<T>) -> State<T>, {
    pub fn new() -> Automaton2D<'a, T, U> {
        Automaton2D {
            grid: Grid2D::new(),
            rule: Rules::new(),
            update_cells: None,
        }
    }

    pub fn set_seed(&mut self, seed: Vec<&'a APoint<'a, i32, State<'a, T>>>){
        self.grid.set_points(seed);
    }
}
