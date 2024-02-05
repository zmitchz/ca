use crate::grid::apoint::APoint;
use crate::grid::grid_2d::Grid2D;
use crate::rule::Rules;
use crate::state::State;

/// struct containing all the cells, rules / transitions, and cells that need 
/// to be updated for the current step 
pub struct Automaton2D<'a, T, U> 
    where 
    U: Fn(State<T>) -> State<T>, {
    grid: Grid2D<'a, APoint<'a, i32, State<'a, T>>, i32>, 
    rules: Rules<State<'a, T>, U>,
    update_cells:  Vec<&'a T>
}

impl<'a, T, U> Automaton2D<'a, T, U>
where
    U: Fn(State<T>) -> State<T>, {
    pub fn new() -> Automaton2D<'a, T, U> {
        Automaton2D {
            grid: Grid2D::new(),
            rules: Rules::new(),
            update_cells: Vec::new(),
        }
    }

    pub fn set_seed(&mut self, seed: Vec<&'a APoint<'a, i32, State<'a, T>>>){
        self.grid.set_points(seed);
    }

    pub fn set_rules(&mut self, ruleset: Rules<State<'a, T>, U>) {
        self.rules = ruleset;
    }

    pub fn add_ucell(&mut self, cell: &'a T) {
        self.update_cells.push(cell);
    }

    pub fn clear_ucells(&mut self) {
        self.update_cells.clear();
    }
}
