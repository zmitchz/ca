/// Automotan Point
 
use crate::{
    grid::point::{IsPoint, Point},
    state::State,
};
use std:: ops::{Add, Mul};

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
    /// # Examples
    ///
    /// ```
    /// use ca::grid::apoint::APoint;
    /// let point: APoint<i32, i32> = APoint::new(10, 10, None);
    /// ```
    pub fn new(x: T, y: T, v: Option<State<'a, U>>) -> APoint<'a, T, U> {
        APoint {
            point: Point::new(x, y),
            state: v,
            prev_state: None,
            neighbours: Vec::new(),
        }
    }

    pub fn cur_state(&self) -> &Option<State<U>> {
        &self.state
    }
    
    pub fn prev_state(&self) -> &Option<State<U>> {
        &self.prev_state
    }

    pub fn set_cur_state(&mut self, state: State<'a, U>) {
        self.state = Some(state);
    }

    pub fn set_prev_state(&mut self, state: State<'a, U>) {
        self.state = Some(state);
    }

    pub fn neighbours(&self) -> &Vec<&'a APoint<'a, T, U>> {
        &self.neighbours
    }

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

