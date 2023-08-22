/*
 * (c) Otso Björklund (2023)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point;
use crate::point_set::set::PointSet;

pub trait PatternMatcher<T: Point> {
    /// Finds occurrences of the given pattern in the point-set and on each found match executes
    /// the given callback. The matches are output as vectors of indices.
    ///
    /// # Arguments
    /// * `query` - The query pattern.
    /// * `point_set` - The point-set from which the occurrences of the query are searched.
    /// * `on_output` - The call back function that is executed on each matching occurrence.
    ///                 The occurrence is given as a vector of indices in the point-set.
    fn find_indices_with_callback(
        &self,
        query: &Pattern<T>,
        point_set: &PointSet<T>,
        on_output: impl FnMut(Vec<usize>),
    );

    /// Finds occurrences of the given pattern in the point-set and returns them as a vector of vectors of indices.
    /// Each vector of indices corresponds to a single found match.
    ///
    /// # Arguments
    /// * `query` - The query pattern.
    /// * `point_set` - The point-set from which the occurrences of the query are searched.
    fn find_indices(&self, query: &Pattern<T>, point_set: &PointSet<T>) -> Vec<Vec<usize>>;

    /// Finds occurrences of the given pattern in the point-set and on each found match executes
    /// the given callback. The matches are output as pattern instances.
    ///
    /// # Arguments
    /// * `query` - The query pattern.
    /// * `point_set` - The point-set from which the occurrences of the query are searched.
    /// * `on_output` - The call back function that is executed on each matching occurrence.
    ///                 The matches are output as pattern instances.
    fn find_occurrences_with_callback(
        &self,
        query: &Pattern<T>,
        point_set: &PointSet<T>,
        on_output: impl FnMut(Pattern<T>),
    );

    /// Finds occurrences of the given pattern in the point-set and returns them as a vector of patterns.
    /// Each returned pattern corresponds to a match.
    ///
    /// # Arguments
    /// * `query` - The query pattern.
    /// * `point_set` - The point-set from which the occurrences of the query are searched.
    fn find_occurrences(&self, query: &Pattern<T>, point_set: &PointSet<T>) -> Vec<Pattern<T>>;
}
