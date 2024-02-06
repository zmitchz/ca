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
    
    /// Return a new `Automaton2D` Struct 
    pub fn new() -> Automaton2D<'a, T, U> {
        Automaton2D {
            grid: Grid2D::new(),
            rules: Rules::new(),
            update_cells: Vec::new(),
        }
    }

    /// Set the initial points grid
    pub fn set_seed(&mut self, seed: Vec<&'a APoint<'a, i32, State<'a, T>>>){
        self.grid.set_points(seed);
    }

    /// set the rules to the struct containing transitions for each state
    pub fn set_rules(&mut self, ruleset: Rules<State<'a, T>, U>) {
        self.rules = ruleset;
    }

    /// Add cell to vector of cells that need to be updated at the next step
    pub fn add_ucell(&mut self, cell: &'a T) {
        self.update_cells.push(cell);
    }

    /// remove all cells from the update Vec, Usually done after executing a 
    /// step
    pub fn clear_ucells(&mut self) {
        self.update_cells.clear();
    }
}
