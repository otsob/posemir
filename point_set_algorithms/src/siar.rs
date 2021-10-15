/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use std::cmp::min;

use crate::mtp_algorithm::MtpAlgorithm;
use crate::point_set::mtp::MTP;
use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point2d;
use crate::point_set::point_set::PointSet;

/// Implements the SIAR algorithm (SIAR for R subdiagonals) for finding a restricted set of MTPs from
/// a point set representation of music (see [Collins 2011]). The implementation
/// is based on the pseudocode in Figure 13.14 of [Meredith 2016].
pub struct SIAR {
    /// The r parameter of algorithm. This defines the number of subdiagonals
    /// computed by the algorithm, i.e., the size of the sliding window.
    r: usize,
}

impl MtpAlgorithm for SIAR {
    /// Computes and returns MTPs restricted by the window size in the given point set.
    ///
    /// # Arguments
    ///
    /// * `point_set` - The point set for which restricted MTPs are computed
    /// * `window` - the size of the window used for restricting the scope of difference vectors
    fn compute_mtps(&self, point_set: &PointSet) -> Vec<MTP> {

        // Compute r subdiagonals
        let n = point_set.len();
        let mut diffs: Vec<(Point2d, usize)> = Vec::with_capacity(n * self.r);

        // Add one to window for convenience in indexing
        let window = self.r + 1;

        for i in 0..n - 1 {
            let from = &point_set[i];
            for j in i + 1..min(n, i + window) {
                let to = &point_set[j];
                diffs.push((to - from, i));
            }
        }

        // Compute the MTP patterns only
        // Sort all differences
        diffs.sort_by(|a, b| { a.0.cmp(&b.0) });

        // Segment the patterns
        let mut mtp_patterns: Vec<Pattern> = Vec::new();
        let m = diffs.len();
        let mut i = 0;
        while i < m {
            let mut indices: Vec<usize> = Vec::new();
            let translator = &diffs[i].0;

            let mut j = i;
            while j < m && *translator == diffs[j].0 {
                indices.push(diffs[j].1);
                j += 1;
            }

            i = j;
            mtp_patterns.push(point_set.get_pattern(&indices));
        }

        // Compute intra-MTP difference vectors
        let mut intra_diffs: Vec<Point2d> = Vec::new();
        for pattern in &mtp_patterns {
            let p = pattern.len();

            for i in 0..p - 1 {
                let from = &pattern[i];
                for j in i + 1..p {
                    intra_diffs.push(&pattern[j] - from);
                }
            }
        }

        // Remove duplicates and compute the intra pattern difference vector
        // frequencies, i.e., number of occurrences.
        intra_diffs.sort();

        let mut intra_diff_freqs: Vec<(Point2d, u64)> = Vec::new();

        let mut current = &intra_diffs[0];
        let mut freq: u64 = 0;

        for diff in &intra_diffs {
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

        // Find the MTPs
        let mut mtps = Vec::new();

        for diff in &intra_diff_freqs {
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
