/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use std::borrow::Borrow;
use std::ops::Index;
use std::slice;

use crate::point_set::point::Point2d;

/// Represents a pattern in a point set.
pub struct Pattern {
    points: Vec<Point2d>,
}

impl Pattern {
    /// Returns a new pattern. The points are copied to the pattern in the order they are given.
    ///
    /// # Arguments
    ///
    /// * `points` - A borrowed vector of points. The returned pattern does not take ownership of these.
    ///
    pub fn new(points: &Vec<&Point2d>) -> Pattern {
        let mut points_copy: Vec<Point2d> = Vec::new();

        for point in points {
            points_copy.push(**point);
        }

        Pattern { points: points_copy }
    }

    /// Returns the number of points in this pattern
    pub fn len(&self) -> usize {
        self.points.len()
    }
}

impl Index<usize> for Pattern {
    type Output = Point2d;

    fn index(&self, index: usize) -> &Self::Output {
        self.points[index].borrow()
    }
}

impl<'a> IntoIterator for &'a Pattern {
    type Item = &'a Point2d;
    type IntoIter = slice::Iter<'a, Point2d>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::point_set::pattern::Pattern;
    use crate::point_set::point::Point2d;

    #[test]
    fn test_constructor_and_access() {
        let mut points = Vec::new();
        let a = Point2d { x: 2.1, y: 0.1 };
        points.push(&a);
        let b = Point2d { x: -1.0, y: 0.0 };
        points.push(&b);
        let c = Point2d { x: -1.0, y: 0.5 };
        points.push(&c);
        let pattern = Pattern::new(&points);

        // Sort the points to test that modifying the original points does not
        // impact the contents of the pattern.
        points.sort();

        assert_eq!(3, pattern.len());
        assert_eq!(a, pattern[0]);
        assert_eq!(b, pattern[1]);
        assert_eq!(c, pattern[2]);
    }

    #[test]
    fn test_iteration() {
        let mut points = Vec::new();
        let a = Point2d { x: 2.1, y: 0.1 };
        points.push(&a);
        let b = Point2d { x: -1.0, y: 0.0 };
        points.push(&b);
        let c = Point2d { x: -1.0, y: 0.5 };
        points.push(&c);
        let d = Point2d { x: -2.0, y: 0.5 };
        points.push(&d);

        let point_set = Pattern::new(&points);

        for (i, point) in point_set.into_iter().enumerate() {
            assert_eq!(*points[i], *point);
        }
    }
}
