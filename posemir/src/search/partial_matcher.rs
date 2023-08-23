/*
 * (c) Otso Björklund (2023)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use crate::discovery::utilities::sort;
use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point;
use crate::point_set::set::PointSet;
use crate::search::pattern_matcher::PatternMatcher;

/// Implements a pattern matcher that finds all partially translationally equivalent occurrences of a pattern
/// from a point-set. Based on the partial matching algorithm presented in
/// [Ukkonen2003] for the problem P2.
pub struct PartialMatcher {
    /// Minimum number of matching points required for a match to be considered a match.
    pub min_match_size: usize,
}

impl<T: Point> PatternMatcher<T> for PartialMatcher {
    fn find_indices_with_callback(
        &self,
        query: &Pattern<T>,
        point_set: &PointSet<T>,
        on_output: impl FnMut(Vec<usize>),
    ) {
        let mut diff_indices = Vec::new();

        for i in 0..query.len() {
            for j in 0..point_set.len() {
                let diff = point_set[j] - query[i];
                diff_indices.push((diff, j));
            }
        }

        sort(&mut diff_indices);
        self.partition(&diff_indices, on_output)
    }
}

impl PartialMatcher {
    /// Partitions the sorted list of difference-index pairs into partial matches exceeding the min_match_size.
    fn partition<T: Point>(&self, diffs: &Vec<(T, usize)>, mut on_output: impl FnMut(Vec<usize>)) {
        let m = diffs.len();
        let mut i = 0;
        while i < m {
            let mut indices: Vec<usize> = Vec::new();
            let translator = diffs[i].0;

            let mut j = i;
            while j < m && translator == diffs[j].0 {
                indices.push(diffs[j].1);
                j += 1;
            }

            i = j;
            if indices.len() >= self.min_match_size {
                on_output(indices);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PartialMatcher;
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
    fn test_given_pattern_in_set_then_exact_matches_found() {
        let point_set = test_point_set();
        let pattern_points = vec![
            &Point2Df64 { x: 0.0, y: 72.0 },
            &Point2Df64 { x: 0.25, y: 74.0 },
            &Point2Df64 { x: 0.5, y: 72.0 },
            &Point2Df64 { x: 0.875, y: 72.0 },
        ];
        let pattern = Pattern::new(&pattern_points);
        let matcher = PartialMatcher { min_match_size: 4 };
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
    fn test_given_pattern_in_set_then_partial_matches_found() {
        let point_set = test_point_set();
        let pattern_points = vec![
            &Point2Df64 { x: -1.0, y: 10.0 },
            &Point2Df64 { x: 0.0, y: 72.0 },
            &Point2Df64 { x: 0.25, y: 74.0 },
            &Point2Df64 { x: 0.5, y: 72.0 },
            &Point2Df64 { x: 0.75, y: 73.0 },
            &Point2Df64 { x: 0.875, y: 72.0 },
        ];
        let pattern = Pattern::new(&pattern_points);
        let matcher = PartialMatcher { min_match_size: 4 };
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
        let matcher = PartialMatcher { min_match_size: 3 };

        assert!(matcher.find_indices(&pattern, &point_set).is_empty());
        assert!(matcher.find_occurrences(&pattern, &point_set).is_empty());
    }
}
