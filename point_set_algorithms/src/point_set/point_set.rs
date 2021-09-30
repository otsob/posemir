/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use std::borrow::Borrow;
use std::ops::Index;
use std::slice;

use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point2d;

/// Represents a sorted set of points.
/// The points in the set are in lexicographical order.
pub struct PointSet {
    points: Vec<Point2d>,
}

impl PointSet {
    /// Returns a point set created from the given points.
    /// The given points do not have to be in any specific order, they are sorted
    /// when the point set is created.
    ///
    /// # Arguments
    ///
    /// * `points` - A vector of points. The returned point set takes ownership of the points.
    ///
    pub fn new(mut points: Vec<Point2d>) -> PointSet {
        points.sort();
        PointSet { points }
    }
}


impl PointSet {
    /// Returns the number of points in this point set
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns a pattern consisting of points at the given indices.
    /// # Arguments
    ///
    /// * `indices` - The indices for the points that form the returned pattern
    pub fn get_pattern(&self, indices: &Vec<usize>) -> Pattern {
        Pattern::new(&indices.iter()
            .map(|i| { &self.points[*i] }).collect::<Vec<&Point2d>>())
    }
}

impl Index<usize> for PointSet {
    type Output = Point2d;

    fn index(&self, index: usize) -> &Self::Output {
        self.points[index].borrow()
    }
}

impl<'a> IntoIterator for &'a PointSet {
    type Item = &'a Point2d;
    type IntoIter = slice::Iter<'a, Point2d>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::point_set::point::Point2d;
    use crate::point_set::point_set::PointSet;

    #[test]
    fn test_constructor_and_access() {
        let mut points = Vec::new();
        points.push(Point2d { x: 2.1, y: 0.1 });
        points.push(Point2d { x: -1.0, y: 0.0 });
        points.push(Point2d { x: -1.0, y: 0.5 });
        let point_set = PointSet::new(points);

        assert_eq!(3, point_set.len());
        assert_eq!(Point2d { x: -1.0, y: 0.0 }, point_set[0]);
        assert_eq!(Point2d { x: -1.0, y: 0.5 }, point_set[1]);
        assert_eq!(Point2d { x: 2.1, y: 0.1 }, point_set[2]);
    }

    #[test]
    fn test_iteration() {
        let mut points = Vec::new();
        points.push(Point2d { x: 2.1, y: 0.1 });
        points.push(Point2d { x: -1.0, y: 0.0 });
        points.push(Point2d { x: -1.0, y: 0.5 });
        points.push(Point2d { x: -2.0, y: 0.5 });

        let mut sorted_points = points.to_vec();
        sorted_points.sort();

        let point_set = PointSet::new(points);

        for (i, point) in point_set.into_iter().enumerate() {
            assert_eq!(sorted_points[i], *point);
        }
    }

    #[test]
    fn test_get_pattern() {
        let mut points = Vec::new();
        points.push(Point2d { x: 2.1, y: 0.1 });
        points.push(Point2d { x: -1.0, y: 0.0 });
        points.push(Point2d { x: -1.0, y: 0.5 });
        points.push(Point2d { x: -2.0, y: 0.5 });

        let mut sorted_points = points.to_vec();
        sorted_points.sort();

        let point_set = PointSet::new(points);

        let pattern = point_set.get_pattern(&vec![0, 3]);
        assert_eq!(2, pattern.len());
        assert_eq!(sorted_points[0], pattern[0]);
        assert_eq!(sorted_points[3], pattern[1]);
    }
}

