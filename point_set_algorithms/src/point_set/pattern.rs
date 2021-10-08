/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use std::borrow::Borrow;
use std::cmp::{min, Ordering};
use std::ops::Index;
use std::slice;

use crate::point_set::point::Point2d;
use crate::point_set::point_set::PointSet;

/// Represents a pattern in a point set.
/// A lexicographical ordering is defined for patterns, so they can easily be sorted lexicographically.
#[derive(Debug)]
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

    /// Returns the vectorized representation of this pattern.
    ///
    /// The vectorized version consists of the differences between the adjacent
    /// points in this pattern. The use for vectorized representations is comparing
    /// the translational equivalence of patterns:
    /// two patterns are translationally equivalent if, and only if, their
    /// vectorized representations are equal.
    pub fn vectorize(&self) -> Pattern {
        let length = self.len();
        let mut diffs = Vec::with_capacity(length - 1);
        for i in 0..length - 1 {
            diffs.push(self[i + 1] - self[i]);
        }

        Pattern { points: diffs }
    }

    /// Returns a translated copy of this pattern
    ///
    /// # Arguments
    ///
    /// * `translator` - The vector by which this pattern is translated.
    pub fn translate(&self, translator: &Point2d) -> Pattern {
        let mut translated_points = Vec::with_capacity(self.len());
        for point in &self.points {
            translated_points.push(point + translator);
        }

        Pattern { points: translated_points }
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

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}

impl Eq for Pattern {}

impl From<PointSet> for Pattern {
    fn from(point_set: PointSet) -> Self {
        Pattern { points: point_set.points() }
    }
}

impl PartialOrd<Self> for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        let mut points_copy = Vec::with_capacity(self.points.len());
        for point in &self.points {
            points_copy.push(*point);
        }

        Pattern { points: points_copy }
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> Ordering {
        let shorter_len = min(self.len(), other.len());

        for i in 0..shorter_len {
            let self_point = self[i];
            let other_point = other[i];

            if self_point < other_point {
                return Ordering::Less;
            }
            if self_point > other_point {
                return Ordering::Greater;
            }
        }

        // If the shared length prefixes of the patterns are equal, then the
        // longer one is greater in lexicographical ordering.
        self.len().cmp(&other.len())
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

    #[test]
    fn test_equality() {
        let mut points = Vec::new();
        let a = Point2d { x: 2.1, y: 0.1 };
        points.push(&a);
        let b = Point2d { x: -1.0, y: 0.0 };
        points.push(&b);
        let c = Point2d { x: -1.0, y: 0.5 };
        points.push(&c);
        let d = Point2d { x: -2.0, y: 0.5 };
        points.push(&d);

        let pattern_a = Pattern::new(&points);
        let pattern_b = Pattern::new(&points);

        assert_eq!(pattern_a, pattern_b);

        let mut more_points = points.to_vec();
        let e = Point2d { x: -1.1, y: 2.6 };
        more_points.push(&e);

        let pattern_c = Pattern::new(&more_points);
        assert_ne!(pattern_a, pattern_c);
    }

    #[test]
    fn test_vectorization_of_single_point_pattern() {
        let mut points = Vec::new();
        let a = Point2d { x: 2.1, y: 0.1 };
        points.push(&a);

        let vectorized = Pattern::new(&points).vectorize();
        assert_eq!(0, vectorized.len());
    }

    #[test]
    fn test_vectorization() {
        let mut points = Vec::new();
        let a = Point2d { x: 2.1, y: 0.1 };
        points.push(&a);
        let b = Point2d { x: -1.0, y: 0.0 };
        points.push(&b);
        let c = Point2d { x: -1.0, y: 0.5 };
        points.push(&c);
        let d = Point2d { x: -2.0, y: 0.5 };
        points.push(&d);

        let vectorized = Pattern::new(&points).vectorize();
        assert_eq!(3, vectorized.len());
        assert_eq!(b - a, vectorized[0]);
        assert_eq!(c - b, vectorized[1]);
        assert_eq!(d - c, vectorized[2]);
    }

    #[test]
    fn test_lex_comparison() {
        let mut points = Vec::new();
        let a = Point2d { x: 2.1, y: 0.1 };
        points.push(&a);
        let b = Point2d { x: -1.0, y: 0.0 };
        points.push(&b);
        let c = Point2d { x: -1.0, y: 0.5 };
        points.push(&c);
        let pattern_a = Pattern::new(&points);

        let mut points = Vec::new();
        let a = Point2d { x: 2.1, y: 0.1 };
        points.push(&a);
        let b = Point2d { x: -1.0, y: 1.0 };
        points.push(&b);
        let pattern_b = Pattern::new(&points);

        assert!(!(pattern_a < pattern_a));
        assert!(pattern_a < pattern_b);
        assert!(!(pattern_a > pattern_b));
    }
}
