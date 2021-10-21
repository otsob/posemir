/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use std::cmp::min;

use crate::mtp_algorithm::MtpAlgorithm;
use crate::point_set::mtp::MTP;
use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point;
use crate::point_set::point_set::PointSet;
use crate::utilities::sort;

/// Implements the SIAR algorithm (SIAR for R subdiagonals) for finding a restricted set of MTPs from
/// a point set representation of music (see [Collins 2011]). The implementation
/// is based on the pseudocode in Figure 13.14 of [Meredith 2016].
pub struct SIAR {
    /// The r parameter of algorithm. This defines the number of subdiagonals
    /// computed by the algorithm, i.e., the size of the sliding window.
    r: usize,
}

impl<T: Point> MtpAlgorithm<T> for SIAR {
    /// Computes and returns MTPs restricted by the window size in the given point set.
    ///
    /// # Arguments
    ///
    /// * `point_set` - The point set for which restricted MTPs are computed
    /// * `window` - the size of the window used for restricting the scope of difference vectors
    fn compute_mtps(&self, point_set: &PointSet<T>) -> Vec<MTP<T>> {
        let forward_diffs = self.compute_differences(point_set);

        let mtp_patterns = SIAR::partition(point_set, &forward_diffs);

        let intra_pattern_diffs = SIAR::compute_intra_pattern_diffs(&mtp_patterns);

        let intra_diff_frequencies = SIAR::compute_diff_frequencies(&intra_pattern_diffs);

        SIAR::compute_mtps(point_set, &intra_diff_frequencies)
    }
}


impl SIAR {
    /// Computes the forward differences with the indices required
    /// for MTP computation.
    /// The forward differences are sorted in ascending lexicographical order.
    fn compute_differences<T: Point>(&self, point_set: &PointSet<T>) -> Vec<(T, usize)> {
        let n = point_set.len();
        let mut diffs: Vec<(T, usize)> = Vec::with_capacity(n * self.r);

        // Add one to window index for convenience in indexing
        let window = self.r + 1;

        for i in 0..n - 1 {
            let from = &point_set[i];
            for j in i + 1..min(n, i + window) {
                let to = &point_set[j];
                diffs.push((*to - *from, i));
            }
        }

        sort(&mut diffs);
        diffs
    }

    /// Partitions the sorted list of difference-index pairs into a MTP patterns. The MTP
    /// difference vectors are not needed in SIAR at this stage.
    fn partition<T: Point>(point_set: &PointSet<T>, forward_diffs: &Vec<(T, usize)>) -> Vec<Pattern<T>> {
        let mut mtp_patterns: Vec<Pattern<T>> = Vec::new();
        let m = forward_diffs.len();
        let mut i = 0;
        while i < m {
            let mut indices: Vec<usize> = Vec::new();
            let translator = &forward_diffs[i].0;

            let mut j = i;
            while j < m && *translator == forward_diffs[j].0 {
                indices.push(forward_diffs[j].1);
                j += 1;
            }

            i = j;
            mtp_patterns.push(point_set.get_pattern(&indices));
        }
        mtp_patterns
    }

    /// Computes the intrapattern diffence vectors (forward differences between points belonging to
    /// same pattern) and sorts them in ascending order.
    fn compute_intra_pattern_diffs<T: Point>(mtp_patterns: &Vec<Pattern<T>>) -> Vec<T> {
        let mut intra_diffs: Vec<T> = Vec::new();
        for pattern in mtp_patterns {
            let p = pattern.len();

            for i in 0..p - 1 {
                let from = &pattern[i];
                for j in i + 1..p {
                    intra_diffs.push(pattern[j] - *from);
                }
            }
        }

        intra_diffs.sort();
        intra_diffs
    }

    /// Computes the frequencies of the difference vectors. Returns a vector of pairs
    /// with the
    /// 0. the difference vector
    /// 1. its number of occurrences in the differences
    ///
    /// Duplication is removed by computing the frequencies. The frequencies are returned in
    /// descending order of frequency: the most frequent difference is first.
    fn compute_diff_frequencies<T: Point>(intra_diffs: &Vec<T>) -> Vec<(T, u64)> {
        let mut intra_diff_freqs: Vec<(T, u64)> = Vec::new();

        let mut current = &intra_diffs[0];
        let mut freq: u64 = 0;

        for diff in intra_diffs {
            if current == diff {
                freq += 1;
            } else {
                intra_diff_freqs.push((*current, freq));
                freq = 1;
                current = diff;
            }
        }
        // Add the last one
        intra_diff_freqs.push((*current, freq));

        // Sort by descending frequency
        intra_diff_freqs.sort_by(|a, b| { b.1.cmp(&a.1) });
        intra_diff_freqs
    }

    /// Computes the MTPs for the intra pattern differences in descending order of size.
    fn compute_mtps<T: Point>(point_set: &PointSet<T>, intra_diff_freqs: &Vec<(T, u64)>) -> Vec<MTP<T>> {
        let mut mtps = Vec::new();

        for diff in intra_diff_freqs {
            let translator = diff.0;
            let intersection = point_set.intersect(&point_set.translate(&(translator * -1.0)));
            mtps.push(MTP { translator, pattern: intersection.into() })
        }

        mtps
    }
}

#[cfg(test)]
mod tests {
    use crate::mtp_algorithm::MtpAlgorithm;
    use crate::point_set::mtp::MTP;
    use crate::point_set::pattern::Pattern;
    use crate::point_set::point::Point2d;
    use crate::point_set::point_set::PointSet;
    use crate::siar::SIAR;

    #[test]
    fn test_minimal_number_of_mtps() {
        // Create a point set where the number of MTPs is minimal.
        let mut points = Vec::new();
        let a = Point2d { x: 1.0, y: 1.0 };
        points.push(a);
        let b = Point2d { x: 2.0, y: 1.0 };
        points.push(b);
        let c = Point2d { x: 3.0, y: 1.0 };
        points.push(c);
        let d = Point2d { x: 4.0, y: 1.0 };
        points.push(d);

        let point_set = PointSet::new(points);
        let siar = SIAR { r: 3 };
        let mut mtps = siar.compute_mtps(&point_set);
        mtps.sort_by(|a, b| { a.translator.cmp(&b.translator) });

        assert_eq!(2, mtps.len());
        assert_eq!(mtps[0], MTP { translator: Point2d { x: 1.0, y: 0.0 }, pattern: Pattern::new(&vec![&a, &b, &c]) });
        assert_eq!(mtps[1], MTP { translator: Point2d { x: 2.0, y: 0.0 }, pattern: Pattern::new(&vec![&a, &b]) });
    }

    #[test]
    fn test_minimal_number_of_mtps_small_window() {
        // Create a point set where the number of MTPs is minimal.
        let mut points = Vec::new();
        let a = Point2d { x: 1.0, y: 1.0 };
        points.push(a);
        let b = Point2d { x: 2.0, y: 1.0 };
        points.push(b);
        let c = Point2d { x: 3.0, y: 1.0 };
        points.push(c);
        let d = Point2d { x: 4.0, y: 1.0 };
        points.push(d);

        let point_set = PointSet::new(points);
        let siar = SIAR { r: 1 };
        let mut mtps = siar.compute_mtps(&point_set);
        mtps.sort_by(|a, b| { a.translator.cmp(&b.translator) });

        assert_eq!(2, mtps.len());
        assert_eq!(mtps[0], MTP { translator: Point2d { x: 1.0, y: 0.0 }, pattern: Pattern::new(&vec![&a, &b, &c]) });
        assert_eq!(mtps[1], MTP { translator: Point2d { x: 2.0, y: 0.0 }, pattern: Pattern::new(&vec![&a, &b]) });
    }
}
