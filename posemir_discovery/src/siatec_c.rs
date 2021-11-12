/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */

use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

use crate::point_set::mtp::Mtp;
use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point;
use crate::point_set::point_set::PointSet;
use crate::point_set::tec::Tec;
use crate::tec_algorithm::TecAlgorithm;
use crate::utilities;

/// Implements the SIATEC-C algorithm (prototype).
pub struct SiatecC {
    /// Maximum allowed inter-onset-interval (IOI) between successive points in a pattern.
    pub max_ioi: f64,
}

impl<T: Point> TecAlgorithm<T> for SiatecC {
    fn compute_tecs(&self, point_set: &PointSet<T>) -> Vec<Tec<T>> {
        let diff_index = self.compute_diff_index(point_set);
        self.compute_mtp_tecs(point_set, &diff_index)
    }
}

impl SiatecC {
    /// Computes the IOI between to points. Onset time is
    /// assumed to be the first component of the points and all points
    /// are assumed to have dimentionality of at least one.
    fn ioi<T: Point>(a: &T, b: &T) -> f64 {
        let a_onset = a.component_f(0);
        let b_onset = b.component_f(0);
        b_onset.unwrap() - a_onset.unwrap()
    }

    /// Returns a vector of difference - index-pair-vector pairs, sorted in ascending lexicographical
    /// order of the difference vectors.
    fn compute_diff_index<T: Point>(&self, point_set: &PointSet<T>) -> Vec<(T, Vec<(usize, usize)>)> {
        let n = point_set.len();

        let forward_diffs = self.compute_forward_diffs(point_set, n);

        SiatecC::partition_by_diff_vector(&forward_diffs)
    }

    fn partition_by_diff_vector<T: Point>(forward_diffs: &Vec<(T, (usize, usize))>) -> Vec<(T, Vec<(usize, usize)>)> {
        let mut diff_index: Vec<(T, Vec<(usize, usize)>)> = Vec::new();
        let m = forward_diffs.len();
        let mut i = 0;
        while i < m {
            let mut index_pairs: Vec<(usize, usize)> = Vec::new();
            let translator = &forward_diffs[i].0;

            let mut j = i;
            while j < m && *translator == forward_diffs[j].0 {
                index_pairs.push(forward_diffs[j].1);
                j += 1;
            }

            diff_index.push((*translator, index_pairs));
            i = j;
        }

        diff_index
    }

    /// Computes forward differences that have inter-onset-interval of at most the limit set
    /// in this instance of SiatecC.
    fn compute_forward_diffs<T: Point>(&self, point_set: &PointSet<T>, n: usize) -> Vec<(T, (usize, usize))> {
        let mut forward_diffs: Vec<(T, (usize, usize))> = Vec::new();

        for i in 0..(n - 1) {
            let from = &point_set[i];

            for j in (i + 1)..n {
                let to = &point_set[j];
                let diff = *to - *from;
                let ioi_opt = diff.component_f(0);
                match ioi_opt {
                    Some(ioi) => { if ioi > self.max_ioi { break; } }
                    None => panic!("Cannot compute with points with no onset component 0")
                }

                forward_diffs.push((diff, (i, j)));
            }
        }

        forward_diffs.sort_by(|a, b| {
            let ordering = a.0.cmp(&b.0);
            if ordering == Equal {
                a.1.1.cmp(&b.1.1)
            } else {
                ordering
            }
        });
        forward_diffs
    }

    fn compute_mtp_tecs<T: Point>(&self, point_set: &PointSet<T>, diff_index: &Vec<(T, Vec<(usize, usize)>)>) -> Vec<Tec<T>> {
        let n = point_set.len();
        // Initialize the window beginnings to start from the points.
        let mut window_begin_indices: Vec<usize> = (0..n).collect();
        let mut tecs = Vec::new();

        while window_begin_indices[0] < n {
            // Compute forward diffs in restricted size window
            let mut forward_diffs = self.compute_forward_diffs_within_window(&point_set, n, &mut window_begin_indices);
            let mtps = SiatecC::partition_to_mtps(point_set, &mut forward_diffs);

            // Split MTP pattern based on max ioi and find translators for subpatterns.
            for mtp in &mtps {
                let split_patterns = SiatecC::split_pattern_on_ioi_gaps(&(*mtp).pattern, self.max_ioi);

                for split_pattern in &split_patterns {
                    let translators;
                    if split_pattern.len() > 1 {
                        translators = SiatecC::find_translators(split_pattern, diff_index, point_set);
                    } else {
                        translators = SiatecC::compute_single_point_translators(split_pattern, point_set);
                    }

                    tecs.push(Tec { pattern: split_pattern.clone(), translators });
                }
            }
        }

        tecs
    }

    fn compute_single_point_translators<T: Point>(pattern: &Pattern<T>, point_set: &PointSet<T>) -> Vec<T> {
        let mut translators = Vec::new();
        let pattern_point = pattern[0];
        for p in point_set {
            if *p != pattern_point {
                translators.push(*p - pattern_point);
            }
        }

        translators
    }

    /// Computes the forward difference vectors for all points, such that, the target points are all within
    /// a restricted size window. Each source point has its own window position, so that difference
    /// vectors of the same size are always computed during the same iteration.
    fn compute_forward_diffs_within_window<T: Point>(&self, point_set: &PointSet<T>, n: usize,
                                                     window_begin_indices: &mut Vec<usize>) -> Vec<(T, usize)> {
        let mut forward_diffs = Vec::new();
        for i in 0..(n - 1) {
            let from = &point_set[i];
            let window_begin_index = window_begin_indices[i];
            if window_begin_index >= n {
                continue;
            }

            let window_begin = &point_set[window_begin_index];
            let mut window_exceeds_data = true;

            for j in window_begin_index..n {
                if i == j {
                    continue;
                }

                let to = &point_set[j];
                let ioi = SiatecC::ioi(window_begin, to);
                let diff: T = *to - *from;

                if ioi > self.max_ioi {
                    window_begin_indices[i] = j;
                    window_exceeds_data = false;
                    break;
                }

                forward_diffs.push((diff, i))
            }

            // If the window has not reached the IOI limit, then the end of the window
            // extends beyond the points in the data set, so there are no mode windows
            // to handle from the starting index.
            if window_exceeds_data {
                window_begin_indices[i] = n;
            }
        }
        forward_diffs
    }

    fn partition_to_mtps<T: Point>(point_set: &PointSet<T>, mut forward_diffs: &mut Vec<(T, usize)>) -> Vec<Mtp<T>> {
        // Sort and partition the diffs to find MTPs
        utilities::sort(&mut forward_diffs);

        let mut mtps: Vec<Mtp<T>> = Vec::new();

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
            mtps.push(Mtp { translator: *translator, pattern: point_set.get_pattern(&indices) });
        }
        mtps
    }

    fn split_pattern_on_ioi_gaps<T: Point>(pattern: &Pattern<T>, max_ioi: f64) -> Vec<Pattern<T>> {
        let mut split_patterns: Vec<Pattern<T>> = Vec::new();
        let mut split = Vec::new();
        let mut prev = &pattern[0];
        for i in 0..pattern.len() {
            let p = &pattern[i];
            let ioi = SiatecC::ioi(prev, p);
            if ioi > max_ioi {
                split_patterns.push(Pattern::new(&split));
                split.clear();
            }
            split.push(p);
            prev = p;
        }

        // Handle any potentially remaining points.
        if !split.is_empty() {
            split_patterns.push(Pattern::new(&split));
        }
        split_patterns
    }

    fn find_indices<'a, T: Point>(diff_index: &'a Vec<(T, Vec<(usize, usize)>)>, translation: &T) -> &'a Vec<(usize, usize)> {
        let index_res = diff_index.binary_search_by(|t| { t.0.cmp(translation) });
        match index_res {
            Ok(index) => &diff_index[index].1,
            Err(index) => {
                print!("Could not find exact match for {:?}, returning closest to {}\n", translation, index);
                if index >= diff_index.len() {
                    return &diff_index[diff_index.len() - 1].1;
                }

                &diff_index[index].1
            }
        }
    }

    fn find_translators<T: Point>(pattern: &Pattern<T>, diff_index: &Vec<(T, Vec<(usize, usize)>)>, point_set: &PointSet<T>) -> Vec<T> {
        let vectorized = pattern.vectorize();
        let v = &vectorized[0];

        let indices = SiatecC::find_indices(diff_index, v);
        let mut target_indices = Vec::with_capacity(indices.len());
        for i in 0..indices.len() {
            target_indices.push(indices[i].1);
        }

        for i in 1..vectorized.len() {
            let diff = &vectorized[i];
            let translatable_indices = SiatecC::find_indices(diff_index, diff);
            let mut tmp_targets = Vec::new();
            let mut j = 0;
            let mut k = 0;

            // Find all indices from which it's possible to continue translating
            // the points by a diff-vector from the pattern's vectorized representation.
            while j < target_indices.len() && k < translatable_indices.len() {
                if target_indices[j] == translatable_indices[k].0 {
                    tmp_targets.push(translatable_indices[k].1);
                    j += 1;
                    k += 1;
                } else if target_indices[j] < translatable_indices[k].0 {
                    j += 1;
                } else if target_indices[j] > translatable_indices[k].0 {
                    k += 1;
                }
            }

            target_indices = tmp_targets;
        }

        let mut translators = Vec::new();
        let last_point = pattern[pattern.len() - 1];
        for i in 0..target_indices.len() {
            let translator = point_set[target_indices[i]] - last_point;
            if !translator.is_zero() {
                translators.push(translator);
            }
        }

        translators
    }

    pub fn remove_translational_duplicates<T: Point>(tecs: &mut Vec<Tec<T>>) {
        tecs.sort_by(|tec_a, tec_b| {
            let a = tec_a.pattern.vectorize();
            let b = tec_b.pattern.vectorize();

            let size_order = a.len().cmp(&b.len());
            if size_order == Ordering::Equal {
                return a.cmp(&b);
            }
            size_order
        });

        tecs.dedup_by(|a, b| { a.pattern.vectorize() == b.pattern.vectorize() })
    }
}


#[cfg(test)]
mod tests {
    use crate::point_set::pattern::Pattern;
    use crate::point_set::point::Point2dF;
    use crate::point_set::point_set::PointSet;
    use crate::point_set::tec::Tec;
    use crate::siatec_c::SiatecC;
    use crate::tec_algorithm::TecAlgorithm;

    #[test]
    fn test_with_minimal_number_of_mtps() {
        // Create a point set where the number of MTPs is minimal.
        let mut points = Vec::new();
        let a = Point2dF { x: 1.0, y: 1.0 };
        points.push(a);
        let b = Point2dF { x: 2.0, y: 1.0 };
        points.push(b);
        let c = Point2dF { x: 3.0, y: 1.0 };
        points.push(c);
        let d = Point2dF { x: 4.0, y: 1.0 };
        points.push(d);

        let point_set = PointSet::new(points);
        let siatec_c = SiatecC { max_ioi: 2.0 };
        let mut tecs = siatec_c.compute_tecs(&point_set);
        tecs.sort_by(|a, b| { a.pattern.len().cmp(&b.pattern.len()) });

        assert_eq!(3, tecs.len());
        assert_eq!(Tec {
            pattern: Pattern::new(&vec![&a]),
            translators: vec![Point2dF { x: 1.0, y: 0.0 },
                              Point2dF { x: 2.0, y: 0.0 },
                              Point2dF { x: 3.0, y: 0.0 }],
        }, tecs[0]);
        assert_eq!(Tec {
            pattern: Pattern::new(&vec![&a, &b]),
            translators: vec![Point2dF { x: 1.0, y: 0.0 },
                              Point2dF { x: 2.0, y: 0.0 }],
        }, tecs[1]);
        assert_eq!(Tec {
            pattern: Pattern::new(&vec![&a, &b, &c]),
            translators: vec![Point2dF { x: 1.0, y: 0.0 }],
        }, tecs[2]);
    }

    #[test]
    fn test_with_gap_and_minimal_number_of_mtps() {
        // Create a point set where the number of MTPs is minimal.
        let mut points = Vec::new();
        let a = Point2dF { x: 1.0, y: 1.0 };
        points.push(a);
        let b = Point2dF { x: 2.0, y: 1.0 };
        points.push(b);
        let c = Point2dF { x: 5.0, y: 1.0 };
        points.push(c);
        let d = Point2dF { x: 6.0, y: 1.0 };
        points.push(d);

        let point_set = PointSet::new(points);
        let siatec_c = SiatecC { max_ioi: 2.0 };
        let mut tecs = siatec_c.compute_tecs(&point_set);

        SiatecC::remove_translational_duplicates(&mut tecs);

        assert_eq!(2, tecs.len());
        assert_eq!(Tec {
            pattern: Pattern::new(&vec![&a]),
            translators: vec![Point2dF { x: 1.0, y: 0.0 },
                              Point2dF { x: 4.0, y: 0.0 },
                              Point2dF { x: 5.0, y: 0.0 }],
        }, tecs[0]);
        assert_eq!(Tec {
            pattern: Pattern::new(&vec![&a, &b]),
            translators: vec![Point2dF { x: 4.0, y: 0.0 }],
        }, tecs[1]);
    }
}