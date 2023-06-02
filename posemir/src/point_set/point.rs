/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops;
use std::ops::{Add, Mul, Sub};

/// Represents a point.
/// Points behave mathematically as vectors: they support addition,
/// subtraction, scalar multiplication, and equality comparisons.
/// Points also support lexicographical sorting.
pub trait Point:
    Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f64, Output = Self>
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Copy
    + Clone
    + Debug
    + Hash
{
    /// Returns true if this point is zero (all components are zero).
    fn is_zero(&self) -> bool;

    /// Returns the component of this point at given index as a float.
    ///
    /// # Arguments
    ///
    /// * `index` - the index of the component to return, or empty if the index is out of bounds
    fn component_f64(&self, index: usize) -> Option<f64>;

    /// Returns the dimensionality of this point.
    fn dimensionality(&self) -> usize;
}

/// Represents a 2-dimensional point/vector with floating point (f64) components.
/// No rounding or inexactness is used in comparisons, so this point type will not work
/// correctly in all cases (e.g., even with music that contains triplets).
#[derive(Debug, Copy)]
pub struct Point2Df64 {
    /// The x coordinate of the point
    pub x: f64,
    /// The y coordinate of the point
    pub y: f64,
}

impl Point for Point2Df64 {
    /// Returns true if this point is zero.
    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    fn component_f64(&self, index: usize) -> Option<f64> {
        if index == 0 {
            Some(self.x)
        } else if index == 1 {
            Some(self.y)
        } else {
            None
        }
    }

    fn dimensionality(&self) -> usize {
        2
    }
}

// Traits for by value arithmetic
impl ops::Add<Point2Df64> for Point2Df64 {
    type Output = Self;

    fn add(self, rhs: Point2Df64) -> Point2Df64 {
        Point2Df64 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Point2Df64> for Point2Df64 {
    type Output = Self;

    fn sub(self, rhs: Point2Df64) -> Self::Output {
        Point2Df64 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for Point2Df64 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Point2Df64 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Traits for by reference arithmetic
impl ops::Add<&Point2Df64> for &Point2Df64 {
    type Output = Point2Df64;

    fn add(self, rhs: &Point2Df64) -> Point2Df64 {
        Point2Df64 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<&Point2Df64> for &Point2Df64 {
    type Output = Point2Df64;

    fn sub(self, rhs: &Point2Df64) -> Self::Output {
        Point2Df64 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for &Point2Df64 {
    type Output = Point2Df64;

    fn mul(self, rhs: f64) -> Self::Output {
        Point2Df64 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Comparisons
impl PartialEq for Point2Df64 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Point2Df64 {
    fn clone(&self) -> Self {
        Point2Df64 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Eq for Point2Df64 {}

impl PartialOrd for Point2Df64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2Df64 {
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

impl Hash for Point2Df64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.x.to_ne_bytes());
        state.write(&self.y.to_ne_bytes());
    }
}

/// Represents a 2-dimensional point/vector with floating point (f64) components.
/// Uses rounding for the x-component (typically used for onset time) in order to avoid issues with
/// tuplet divisions that are not precisely expressible with floating point numbers.
#[derive(Debug, Copy)]
pub struct Point2DRf64 {
    /// The rounded x coordinate of the point
    pub rounded_x: f64,
    /// The y coordinate of the point
    pub y: f64,

    /// Raw unrounded x component used for computations in order to avoid accumulating rounding errors.
    raw_x: f64,
}

impl Point2DRf64 {
    const PRECISION: f64 = 100000.0;

    fn round(number: f64) -> f64 {
        (number * Point2DRf64::PRECISION).round() / Point2DRf64::PRECISION
    }

    pub fn new(raw_x: f64, y: f64) -> Point2DRf64 {
        Point2DRf64 {
            rounded_x: Point2DRf64::round(raw_x),
            y,
            raw_x,
        }
    }
}

impl Point for Point2DRf64 {
    /// Returns true if this point is zero.
    fn is_zero(&self) -> bool {
        self.rounded_x == 0.0 && self.y == 0.0
    }

    fn component_f64(&self, index: usize) -> Option<f64> {
        if index == 0 {
            Some(self.rounded_x)
        } else if index == 1 {
            Some(self.y)
        } else {
            None
        }
    }

    fn dimensionality(&self) -> usize {
        2
    }
}

// Traits for by value arithmetic
impl ops::Add<Point2DRf64> for Point2DRf64 {
    type Output = Self;

    fn add(self, rhs: Point2DRf64) -> Point2DRf64 {
        let raw_x = self.raw_x + rhs.raw_x;

        Point2DRf64 {
            rounded_x: Point2DRf64::round(raw_x),
            y: self.y + rhs.y,
            raw_x,
        }
    }
}

impl ops::Sub<Point2DRf64> for Point2DRf64 {
    type Output = Self;

    fn sub(self, rhs: Point2DRf64) -> Self::Output {
        let raw_x = self.raw_x - rhs.raw_x;

        Point2DRf64 {
            rounded_x: Point2DRf64::round(raw_x),
            y: self.y - rhs.y,
            raw_x,
        }
    }
}

impl ops::Mul<f64> for Point2DRf64 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let raw_x = self.raw_x * rhs;

        Point2DRf64 {
            rounded_x: Point2DRf64::round(raw_x),
            y: self.y * rhs,
            raw_x,
        }
    }
}

// Traits for by reference arithmetic
impl ops::Add<&Point2DRf64> for &Point2DRf64 {
    type Output = Point2DRf64;

    fn add(self, rhs: &Point2DRf64) -> Point2DRf64 {
        let raw_x = self.raw_x + rhs.raw_x;

        Point2DRf64 {
            rounded_x: Point2DRf64::round(raw_x),
            y: self.y + rhs.y,
            raw_x,
        }
    }
}

impl ops::Sub<&Point2DRf64> for &Point2DRf64 {
    type Output = Point2DRf64;

    fn sub(self, rhs: &Point2DRf64) -> Self::Output {
        let raw_x = self.raw_x - rhs.raw_x;

        Point2DRf64 {
            rounded_x: Point2DRf64::round(raw_x),
            y: self.y - rhs.y,
            raw_x,
        }
    }
}

impl ops::Mul<f64> for &Point2DRf64 {
    type Output = Point2DRf64;

    fn mul(self, rhs: f64) -> Self::Output {
        let raw_x = self.raw_x * rhs;

        Point2DRf64 {
            rounded_x: Point2DRf64::round(raw_x),
            y: self.y * rhs,
            raw_x,
        }
    }
}

// Comparisons
impl PartialEq for Point2DRf64 {
    fn eq(&self, other: &Self) -> bool {
        self.rounded_x == other.rounded_x && self.y == other.y
    }
}

impl Clone for Point2DRf64 {
    fn clone(&self) -> Self {
        Point2DRf64 {
            rounded_x: self.rounded_x,
            y: self.y,
            raw_x: self.raw_x,
        }
    }
}

impl Eq for Point2DRf64 {}

impl PartialOrd for Point2DRf64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2DRf64 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rounded_x < other.rounded_x {
            return Ordering::Less;
        }

        if self.rounded_x > other.rounded_x {
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

impl Hash for Point2DRf64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.rounded_x.to_ne_bytes());
        state.write(&self.y.to_ne_bytes());
    }
}

/// Represents a 2-dimensional point/vector with integer components.
#[derive(Debug, Copy)]
pub struct Point2Di64 {
    /// The x coordinate of the point
    pub x: i64,
    /// The y coordinate of the point
    pub y: i64,
}

impl Point for Point2Di64 {
    /// Returns true if this point is zero.
    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn component_f64(&self, index: usize) -> Option<f64> {
        if index == 0 {
            Some(self.x as f64)
        } else if index == 1 {
            Some(self.y as f64)
        } else {
            None
        }
    }

    fn dimensionality(&self) -> usize {
        2
    }
}

// Traits for by value arithmetic
impl ops::Add<Point2Di64> for Point2Di64 {
    type Output = Self;

    fn add(self, rhs: Point2Di64) -> Point2Di64 {
        Point2Di64 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Point2Di64> for Point2Di64 {
    type Output = Self;

    fn sub(self, rhs: Point2Di64) -> Self::Output {
        Point2Di64 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for Point2Di64 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs_int = rhs as i64;
        Point2Di64 {
            x: self.x * rhs_int,
            y: self.y * rhs_int,
        }
    }
}

// Traits for by reference arithmetic
impl ops::Add<&Point2Di64> for &Point2Di64 {
    type Output = Point2Di64;

    fn add(self, rhs: &Point2Di64) -> Point2Di64 {
        Point2Di64 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<&Point2Di64> for &Point2Di64 {
    type Output = Point2Di64;

    fn sub(self, rhs: &Point2Di64) -> Self::Output {
        Point2Di64 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for &Point2Di64 {
    type Output = Point2Di64;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs_int = rhs as i64;
        Point2Di64 {
            x: self.x * rhs_int,
            y: self.y * rhs_int,
        }
    }
}

// Comparisons
impl PartialEq for Point2Di64 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Point2Di64 {
    fn clone(&self) -> Self {
        Point2Di64 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Eq for Point2Di64 {}

impl PartialOrd for Point2Di64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2Di64 {
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

impl Hash for Point2Di64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i64(self.x);
        state.write_i64(self.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(Point2Df64 { x: 3.0, y: 1.0 }, Point2Df64 { x: 3.0, y: 1.0 });
        assert_ne!(Point2Df64 { x: 3.0, y: 1.0 }, Point2Df64 { x: 3.0, y: 2.0 });

        assert_eq!(Point2DRf64::new(3.0, 1.0), Point2DRf64::new(3.0, 1.0));
        assert_eq!(
            Point2DRf64::new(1.0, 1.0),
            Point2DRf64::new(0.33333333333 * 3.0, 1.0)
        );
        assert_ne!(Point2DRf64::new(3.0, 1.0), Point2DRf64::new(3.0, 2.0));

        assert_ne!(Point2Di64 { x: 3, y: 1 }, Point2Di64 { x: 3, y: 2 });
    }

    #[test]
    fn test_add() {
        let a = Point2Df64 { x: 1.0, y: 1.0 };
        let b = Point2Df64 { x: 2.0, y: 0.0 };
        assert_eq!(Point2Df64 { x: 3.0, y: 1.0 }, a + b);

        let a = Point2DRf64::new(1.0, 1.0);
        let b = Point2DRf64::new(2.0, 0.0);
        assert_eq!(Point2DRf64::new(3.0, 1.0), a + b);

        let a = Point2Di64 { x: 1, y: 1 };
        let b = Point2Di64 { x: 2, y: 0 };
        assert_eq!(Point2Di64 { x: 3, y: 1 }, a + b);
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Point2Df64 { x: -1.0, y: 1.0 },
            Point2Df64 { x: 1.0, y: 2.0 } - Point2Df64 { x: 2.0, y: 1.0 }
        );

        assert_eq!(
            Point2DRf64::new(-1.0, 1.0),
            Point2DRf64::new(1.0, 2.0) - Point2DRf64::new(2.0, 1.0)
        );

        assert_eq!(
            Point2Di64 { x: -1, y: 1 },
            Point2Di64 { x: 1, y: 2 } - Point2Di64 { x: 2, y: 1 }
        );
    }

    #[test]
    fn test_cmp_floats() {
        let a = Point2Df64 { x: -1.0, y: 0.0 };
        let b = Point2Df64 { x: -0.5, y: 0.0 };
        let c = Point2Df64 { x: -0.5, y: 1.0 };

        assert_eq!(Some(Ordering::Equal), a.partial_cmp(&a));
        assert_eq!(Some(Ordering::Less), a.partial_cmp(&b));
        assert_eq!(Some(Ordering::Greater), b.partial_cmp(&a));
        assert!(a <= a);
        assert!(a < b);
        assert!(b < c);
        assert!(c > a);
    }

    #[test]
    fn test_cmp_rounding_floats() {
        let a = Point2DRf64::new(-1.0, 0.0);
        let b = Point2DRf64::new(-0.5, 0.0);
        let c = Point2DRf64::new(-0.5, 1.0);
        let d = Point2DRf64::new(-1.0000000000000000000001, 0.0);

        assert_eq!(Some(Ordering::Equal), a.partial_cmp(&a));
        assert_eq!(Some(Ordering::Equal), a.partial_cmp(&d));
        assert_eq!(Some(Ordering::Less), a.partial_cmp(&b));
        assert_eq!(Some(Ordering::Greater), b.partial_cmp(&a));
        assert!(a <= a);
        assert!(a < b);
        assert!(b < c);
        assert!(c > a);
    }

    #[test]
    fn test_cmp_ints() {
        let a = Point2Di64 { x: -1, y: 0 };
        let b = Point2Di64 { x: -0, y: 0 };
        let c = Point2Di64 { x: -0, y: 1 };

        assert_eq!(Some(Ordering::Equal), a.partial_cmp(&a));
        assert_eq!(Some(Ordering::Less), a.partial_cmp(&b));
        assert_eq!(Some(Ordering::Greater), b.partial_cmp(&a));
        assert!(a <= a);
        assert!(a < b);
        assert!(b < c);
        assert!(c > a);
    }

    #[test]
    fn test_component_access() {
        let a = Point2Df64 { x: 1.0, y: 2.0 };
        assert_eq!(2, a.dimensionality());
        assert_eq!(Some(1.0), a.component_f64(0));
        assert_eq!(Some(2.0), a.component_f64(1));
        assert_eq!(None, a.component_f64(3));

        let b = Point2Di64 { x: 1, y: 2 };
        assert_eq!(2, b.dimensionality());
        assert_eq!(Some(1.0), b.component_f64(0));
        assert_eq!(Some(2.0), b.component_f64(1));
        assert_eq!(None, b.component_f64(3));

        let c = Point2DRf64::new(1.0, 2.0);
        assert_eq!(2, c.dimensionality());
        assert_eq!(Some(1.0), c.component_f64(0));
        assert_eq!(Some(2.0), c.component_f64(1));
        assert_eq!(None, c.component_f64(3));
    }
}
