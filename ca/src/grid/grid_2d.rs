use std::ops::{Add, Mul};
use std::marker::PhantomData;

use super::point::IsPoint;


/// A 2 dimensional grid that can store anything that implements the 
/// [`IsPoint`] Trait
pub struct Grid2D<'a, T, U>
where
        T: IsPoint<U>,
        U: Copy + Ord + PartialEq + Add<Output = U> + Mul<Output = U>, {
        points: Vec<&'a T>,
        phantom: PhantomData<&'a U>,
    }

impl<'a, T, U> Grid2D<'a, T, U>
where
    T: IsPoint<U>,
    U: Copy + Ord + PartialEq + Add<Output = U> + Mul<Output = U>, {
    pub fn new() -> Grid2D<'a, T, U> {
        Grid2D{
            points: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn points(&self) -> &Vec<&'a T> {
        &self.points
    }

    pub fn push(&mut self, p: &'a T) {
        self.points.push(p);
    }

    pub fn set_points(&mut self, p: Vec<&'a T>){
        self.points = p;
    }

}

#[cfg(test)]
mod tests {
    use crate::grid::grid_2d::*;
    use crate::grid::point::APoint;

    // #[test]
    // fn new_empty_test() {
    //     let grid: Grid2D<i32> = Grid2D::new_empty(5, 5);
    //     println!("{}", grid.to_string());
    // }
}
