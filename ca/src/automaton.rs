use std::fmt::Display;
use std::ops::{Add, Mul};

use crate::grid::grid_2d::Grid2D;
use crate::grid::point::IsPoint;
use crate::rule::Rules;

/// struct containing all the cells, rules / transitions, and cells that need 
/// to be updated for the current step 
pub struct Automaton2D<'a, T, U, V>
    where
    T: IsPoint<U>,
    U: Display + Copy + Ord + PartialEq + Add<Output = U> + Mul<Output = U>,
    V: Fn(V) -> U {
    grid: Grid2D<'a, T, U>,
    rule: Rules<V, U>,
    update_cells:  Option<Vec<&'a T>>
}

impl<'a, T, U, V> Automaton2D<'a, T, U, V>
where
    T: IsPoint<U>,
    U: Display + Copy + Ord + PartialEq + Add<Output = U> + Mul<Output = U>,
    V: Fn(V) -> U {
    pub fn new() -> Automaton2D<'a, T, U, V> {
        Automaton2D {
            grid: Grid2D::new(),
            rule: Rules::new(),
            update_cells: None,
        }
    }

    pub fn set_seed(&mut self, seed: Vec<&'a T>){
        self.grid.set_points(seed);
    }
}
