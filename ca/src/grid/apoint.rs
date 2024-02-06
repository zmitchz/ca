/// Automotan Point
 
use crate::{
    grid::point::{IsPoint, Point},
    state::State,
};
use std:: ops::{Add, Mul};

/// Automotan Point. contains a point along with neighbourhood and states
pub struct APoint<'a, T, U>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>, {
    point: Point<T>,
    state: Option<State<'a, U>>,
    prev_state: Option<State<'a, U>>,
    neighbours: Vec<&'a APoint<'a, T, U>>,
}

impl<'a, T, U> APoint<'a, T, U>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>, {
    /// Create a new `APoint`
    ///
    /// # Arguments
    ///
    /// * `x` - x coord of point
    ///
    /// * `y` - y coord of point
    ///
    /// * `v` - Value to be placed inside of the [`APoint`] wrapped in some, None
    ///         if No value to be assigned at creation
    ///
    pub fn new(x: T, y: T, v: Option<State<'a, U>>) -> APoint<'a, T, U> {
        APoint {
            point: Point::new(x, y),
            state: v,
            prev_state: None,
            neighbours: Vec::new(),
        }
    }

    /// Get current state wrapped in option
    pub fn cur_state(&self) -> &Option<State<U>> {
        &self.state
    }
    
    /// Get previous state wrapped in option
    pub fn prev_state(&self) -> &Option<State<U>> {
        &self.prev_state
    }

    /// Set current state
    pub fn set_cur_state(&mut self, state: State<'a, U>) {
        self.state = Some(state);
    }

    /// Set previous state
    pub fn set_prev_state(&mut self, state: State<'a, U>) {
        self.state = Some(state);
    }

    /// Get vector containing pointer to neighbours
    pub fn neighbours(&self) -> &Vec<&'a APoint<'a, T, U>> {
        &self.neighbours
    }

    /// Set neighbourhood Vec
    pub fn set_neighbours(&mut self, n: Vec<&'a APoint<'a, T, U>>) {
        self.neighbours = n;
    }
}

impl<'a, T, U> IsPoint<T> for APoint<'a, T, U>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>, {
    fn x(&self) -> T {
        self.point.x()
    }
    fn y(&self) -> T {
        self.point.y()
    }
}

