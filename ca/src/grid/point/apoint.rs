/// Automotan Point
 
use super::{IsPoint, VPoint};
use std::{
    fmt::Display,
    ops::{Add, Mul},
};


#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct APoint<'a, T, U>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T> + Display,
    U: Display + Ord + PartialOrd + PartialEq, {
    name: String,
    point: VPoint<T, U>,
    prev_state: Option<U>,
    neighbours: Vec<&'a APoint<'a, T, U>>,
}

impl<'a, T, U> APoint<'a, T, U>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T> + Display,
    U: Display + Ord + PartialOrd + PartialEq, {
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
    /// use ca::grid::point::APoint;
    /// let point: APoint<i32, i32> = APoint::new(10, 10, None, "p1".to_string());
    /// ```
    pub fn new(x: T, y: T, v: Option<U>, name: String) -> APoint<'a, T, U> {
        APoint {
            name,
            point: VPoint::new(x, y, v),
            prev_state: None,
            neighbours: Vec::new(),
        }
    }

    pub fn cur_state(&self) -> &Option<U> {
        self.point.val()
    }
    
    pub fn prev_state(&self) -> &Option<U> {
        &self.prev_state
    }

    pub fn neighbours(&self) -> &Vec<&'a APoint<'a, T, U>> {
        &self.neighbours
    }

    pub fn set_neighbours(&mut self, n: Vec<&'a APoint<'a, T, U>>) {
        self.neighbours = n;
    }
}

impl<'a, T, U> Display for APoint<'a, T, U>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T> + Display,
    U: Display + Ord + PartialOrd + PartialEq, {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n = "".to_string();
        for p in self.neighbours(){
            n += &(p.to_string() + "\n");
        }
        write!(f, "x: {}, y: {}\nval: {}\nneighbours: {}", self.x(), self.y(), self.name, n)
    }
}
impl<'a, T, U> IsPoint<T> for APoint<'a, T, U>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T> + Display,
    U: Display + Ord + PartialOrd + PartialEq, {
    fn x(&self) -> T {
        self.point.x()
    }
    fn y(&self) -> T {
        self.point.y()
    }
}

