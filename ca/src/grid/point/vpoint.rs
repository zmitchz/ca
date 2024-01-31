use crate::grid::point::{IsPoint, Point};
use std::ops::{Add, Mul};

/// Point that can store a value along with coordinates
#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct VPoint<T, U>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
    U: Ord + PartialOrd + PartialEq,
{
    point: Point<T>,
    val: Option<U>,
}

impl<T, U> VPoint<T, U>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
    U: Ord + PartialOrd + PartialEq,
{
    /// Create a new `VPoint`
    ///
    /// # Arguments
    ///
    /// * `x` - x coord of point
    ///
    /// * `y` - y coord of point
    ///
    /// * `v` - Value to be placed inside of the `VPoint`
    /// # Examples
    ///
    /// ```
    /// use ca::grid::point::VPoint;
    /// let point = VPoint::new(10, 10, Some("jim"));
    /// ```
    pub fn new(x: T, y: T, v: Option<U>) -> VPoint<T, U> {
        VPoint {
            point: Point::new(x, y),
            val: v,
        }
    }
    pub fn val(&self) -> &Option<U> {
        &self.val
    }
    pub fn set_val(&mut self, new_val: U) {
        self.val = Some(new_val)
    }

    pub fn point(&self) -> &Point<T> {
        &self.point
    }
}

impl<T, U> IsPoint<T> for VPoint<T, U>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
    U: Ord + PartialOrd + PartialEq,
{
    fn x(&self) -> T {
        self.point.x()
    }
    fn y(&self) -> T {
        self.point.y()
    }
}
