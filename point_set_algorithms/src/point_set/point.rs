/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use std::cmp::Ordering;
use std::ops;
use std::ops::{Add, Mul, Sub};

/// Represents a point.
/// Points behave mathematically as vectors: they support addition,
/// subtraction, scalar multiplication, and equality comparisons.
/// Points also support lexicographical sorting.
pub trait Point:
Sized
+ Add<Self, Output=Self>
+ Sub<Self, Output=Self>
+ Mul<f64, Output=Self>
+ PartialEq
+ Eq
+ PartialOrd
+ Ord
+ Copy
+ Clone
{
    /// Returns true if this point is zero (all components are zero).
    fn is_zero(&self) -> bool;
}


/// Represents a 2-dimensional point/vector with floating point components.
#[derive(Debug, Copy)]
pub struct Point2dF {
    /// The x coordinate of the point
    pub x: f64,
    /// The y coordinate of the point
    pub y: f64,
}

impl Point for Point2dF {
    /// Returns true if this point is zero.
    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

// Traits for by value arithmetic
impl ops::Add<Point2dF> for Point2dF {
    type Output = Point2dF;

    fn add(self, rhs: Point2dF) -> Point2dF {
        Point2dF { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<Point2dF> for Point2dF {
    type Output = Point2dF;

    fn sub(self, rhs: Point2dF) -> Self::Output {
        Point2dF { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::Mul<f64> for Point2dF {
    type Output = Point2dF;

    fn mul(self, rhs: f64) -> Self::Output {
        Point2dF { x: self.x * rhs, y: self.y * rhs }
    }
}

// Traits for by reference arithmetic
impl ops::Add<&Point2dF> for &Point2dF {
    type Output = Point2dF;

    fn add(self, rhs: &Point2dF) -> Point2dF {
        Point2dF { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<&Point2dF> for &Point2dF {
    type Output = Point2dF;

    fn sub(self, rhs: &Point2dF) -> Self::Output {
        Point2dF { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::Mul<f64> for &Point2dF {
    type Output = Point2dF;

    fn mul(self, rhs: f64) -> Self::Output {
        Point2dF { x: self.x * rhs, y: self.y * rhs }
    }
}

// Comparisons
impl PartialEq for Point2dF {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Point2dF {
    fn clone(&self) -> Self {
        Point2dF { x: self.x, y: self.y }
    }
}

impl Eq for Point2dF {}

impl PartialOrd for Point2dF {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2dF {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x < other.x {
            return Ordering::Less;
        }

        if self.x > other.x {
            return Ordering::Greater;
        }

        if self.y < other.y {
            return Ordering::Less;
        }

        if self.y > other.y {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(Point2dF { x: 3.0, y: 1.0 }, Point2dF { x: 3.0, y: 1.0 });
        assert_ne!(Point2dF { x: 3.0, y: 1.0 }, Point2dF { x: 3.0, y: 2.0 });
    }

    #[test]
    fn test_add() {
        let a = Point2dF { x: 1.0, y: 1.0 };
        let b = Point2dF { x: 2.0, y: 0.0 };
        assert_eq!(Point2dF { x: 3.0, y: 1.0 }, a + b);
    }

    #[test]
    fn test_sub() {
        assert_eq!(Point2dF { x: -1.0, y: 1.0 }, Point2dF { x: 1.0, y: 2.0 } - Point2dF { x: 2.0, y: 1.0 });
    }

    #[test]
    fn test_cmp() {
        let a = Point2dF { x: -1.0, y: 0.0 };
        let b = Point2dF { x: -0.5, y: 0.0 };
        let c = Point2dF { x: -0.5, y: 1.0 };

        assert_eq!(Some(Ordering::Equal), a.partial_cmp(&a));
        assert_eq!(Some(Ordering::Less), a.partial_cmp(&b));
        assert_eq!(Some(Ordering::Greater), b.partial_cmp(&a));
        assert!(a <= a);
        assert!(a < b);
        assert!(b < c);
        assert!(c > a);
    }
}
