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

/// Implements a pattern matcher that finds all translationally equivalent occurrences of a pattern
/// from a point-set. Based on the exact matching algorithm presented in
/// [Ukkonen2003] for the problem P1.
pub struct ExactMatcher {}

impl<T: Point> PatternMatcher<T> for ExactMatcher {
    fn find_indices_with_callback(
        &self,
        query: &Pattern<T>,
        point_set: &PointSet<T>,
        mut on_output: impl FnMut(Vec<usize>),
    ) {
        for i in 0..(point_set.len() - query.len() + 1) {
            let mut candidate = Vec::with_capacity(query.len());
            let translator = point_set[i] - query[0];
            let cutoff_point = query[query.len() - 1] + translator;

            let mut scan_index = i;
            let mut query_index = 0;

            while scan_index < point_set.len()
                && query_index < query.len()
                && point_set[scan_index] <= cutoff_point
            {
                let translated_query_point = query[query_index] + translator;

                if point_set[scan_index] == translated_query_point {
                    candidate.push(scan_index);
                }

                if translated_query_point <= point_set[scan_index] {
                    query_index += 1;
                }

                scan_index += 1;
            }

            if candidate.len() == query.len() {
                on_output(candidate);
            }
        }
    }

    fn find_indices(&self, query: &Pattern<T>, point_set: &PointSet<T>) -> Vec<Vec<usize>> {
        let mut occurrences = Vec::new();
        let on_output = |occurrence: Vec<usize>| occurrences.push(occurrence);
        self.find_indices_with_callback(query, point_set, on_output);
        occurrences
    }

    fn find_occurrences_with_callback(
        &self,
        query: &Pattern<T>,
        point_set: &PointSet<T>,
        mut on_output: impl FnMut(Pattern<T>),
    ) {
        let on_index_output = |occ_ind: Vec<usize>| {
            let occurrence = point_set.get_pattern(&occ_ind);
            on_output(occurrence);
        };

        self.find_indices_with_callback(query, point_set, on_index_output);
    }

    fn find_occurrences(&self, query: &Pattern<T>, point_set: &PointSet<T>) -> Vec<Pattern<T>> {
        let mut occurrences = Vec::new();
        let on_output = |patt: Pattern<T>| occurrences.push(patt);
        self.find_occurrences_with_callback(query, point_set, on_output);

        occurrences
    }
}

#[cfg(test)]
mod tests {
    use super::ExactMatcher;
    use super::PatternMatcher;
    use crate::point_set::pattern::Pattern;
    use crate::point_set::point::Point2Df64;
    use crate::point_set::set::PointSet;

    fn test_point_set() -> PointSet<Point2Df64> {
        let points = vec![
            Point2Df64 { x: 0.0, y: 72.0 },
            Point2Df64 { x: 0.25, y: 74.0 },
            Point2Df64 { x: 0.5, y: 72.0 },
            Point2Df64 { x: 0.875, y: 72.0 },
            Point2Df64 { x: 1.0, y: 45.0 },
            Point2Df64 { x: 1.0, y: 60.0 },
            Point2Df64 { x: 1.25, y: 47.0 },
            Point2Df64 { x: 1.25, y: 62.0 },
            Point2Df64 { x: 1.5, y: 45.0 },
            Point2Df64 { x: 1.875, y: 45.0 },
        ];

        PointSet::new(points)
    }

    #[test]
    fn test_given_pattern_in_set_then_matches_found() {
        let point_set = test_point_set();
        let pattern_points = vec![
            &Point2Df64 { x: 0.0, y: 72.0 },
            &Point2Df64 { x: 0.25, y: 74.0 },
            &Point2Df64 { x: 0.5, y: 72.0 },
            &Point2Df64 { x: 0.875, y: 72.0 },
        ];
        let pattern = Pattern::new(&pattern_points);
        let matcher = ExactMatcher {};
        let indices = matcher.find_indices(&pattern, &point_set);

        assert_eq!(2, indices.len());
        assert_eq!(vec![0, 1, 2, 3], indices[0]);
        assert_eq!(vec![4, 6, 8, 9], indices[1]);

        let occurrences = matcher.find_occurrences(&pattern, &point_set);
        assert_eq!(2, occurrences.len());
        assert_eq!(
            Pattern::new(&vec![
                &Point2Df64 { x: 0.0, y: 72.0 },
                &Point2Df64 { x: 0.25, y: 74.0 },
                &Point2Df64 { x: 0.5, y: 72.0 },
                &Point2Df64 { x: 0.875, y: 72.0 },
            ]),
            occurrences[0]
        );
        assert_eq!(
            Pattern::new(&vec![
                &Point2Df64 { x: 1.0, y: 45.0 },
                &Point2Df64 { x: 1.25, y: 47.0 },
                &Point2Df64 { x: 1.5, y: 45.0 },
                &Point2Df64 { x: 1.875, y: 45.0 },
            ]),
            occurrences[1]
        )
    }

    #[test]
    fn test_given_pattern_not_in_set_then_no_matches_found() {
        let point_set = test_point_set();
        let pattern_points = vec![
            &Point2Df64 { x: 0.0, y: 72.0 },
            &Point2Df64 { x: 0.25, y: 74.0 },
            &Point2Df64 { x: 0.375, y: 72.0 },
        ];
        let pattern = Pattern::new(&pattern_points);
        let matcher = ExactMatcher {};

        assert!(matcher.find_indices(&pattern, &point_set).is_empty());
        assert!(matcher.find_occurrences(&pattern, &point_set).is_empty());
    }
}
