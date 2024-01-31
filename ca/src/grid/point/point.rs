use std::ops::{Add, Mul};

/// impl on anything that can be represented by its x and y coords
pub trait IsPoint<T>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>
{
    fn x(&self) -> T;
    fn y(&self) -> T;
}

/// Basic point struct containing only an x and y coordinate
#[derive(Clone, Copy, Eq, PartialOrd, Ord, PartialEq, Debug)]
pub struct Point<T>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    pub fn new(x: T, y: T) -> Point<T> {
         Point { x: x.clone(), y: y.clone() } }

}

impl<T> IsPoint<T> for Point<T>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    fn x(&self) -> T { self.x }
    fn y(&self) -> T { self.y }
}

impl<T> Add for Point<T>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(
            self.x + rhs.x,
            self.y + rhs.y,
        )
    }
}

impl<T> Add<(T, T)> for Point<T>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: (T, T)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl<T> Add<T> for Point<T>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T> Mul for Point<T>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Mul<(T, T)> for Point<T>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: (T, T)) -> Self::Output {
        Self {
            x: self.x * rhs.0,
            y: self.y * rhs.1,
        }
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Copy + Ord + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_point_num_test(){
        let point_a = Point::new(10, 10);
        assert_eq!(point_a + 10, Point::new(20, 20));
    }

    #[test]
    fn add_point_point_test(){
        let point_a = Point::new(10, 10);
        let point_b = Point::new(10, 10);
        assert_eq!(point_a + point_b, Point::new(20, 20));
    }

    #[test]
    fn add_point_tuple_test(){
        let point_a = Point::new(10, 10);
        let tuple_a = (10, 10);
        assert_eq!(point_a + tuple_a, Point::new(20, 20));
    }

    #[test]
    fn mult_point_num_test(){
        let point_a = Point::new(10, 10);
        assert_eq!(point_a * 10, Point::new(100, 100));
    }

    #[test]
    fn mult_point_point_test(){
        let point_a = Point::new(10, 10);
        let point_b = Point::new(10, 10);
        assert_eq!(point_a * point_b, Point::new(100, 100));
    }

    #[test]
    fn mult_point_tuple_test(){
        let point_a = Point::new(10, 10);
        let tuple_a = (10, 10);
        assert_eq!(point_a * tuple_a, Point::new(100, 100));
    }

}
