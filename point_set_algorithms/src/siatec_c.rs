/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */

use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

use crate::point_set::mtp::MTP;
use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point;
use crate::point_set::point_set::PointSet;
use crate::point_set::tec::TEC;
use crate::tec_algorithm::TecAlgorithm;
use crate::utilities;

/// Implements the SIATEC-C algorithm (prototype).
struct SiatecC {
    /// Maximum allowed inter-onset-interval (IOI) between points in a pattern.
    pub max_ioi: f64,
}

impl<T: Point> TecAlgorithm<T> for SiatecC {
    fn compute_tecs(&self, point_set: &PointSet<T>) -> Vec<TEC<T>> {
        let diff_index = self.compute_diff_index(point_set);
        self.compute_mtp_tecs(point_set, &diff_index)
    }
}

impl SiatecC {
    fn ioi<T: Point>(a: &T, b: &T) -> f64 {
        let a_first = a.component_f(0);
        let b_first = b.component_f(0);
        b_first.unwrap() - a_first.unwrap()
    }

    fn compute_diff_index<T: Point>(&self, point_set: &PointSet<T>) -> Vec<(T, Vec<(usize, usize)>)> {
        let mut forward_diffs: Vec<(T, (usize, usize))> = Vec::new();
        let n = point_set.len();

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

    fn compute_mtp_tecs<T: Point>(&self, point_set: &PointSet<T>, diff_index: &Vec<(T, Vec<(usize, usize)>)>) -> Vec<TEC<T>> {
        let n = point_set.len();
        // Initialize the window beginnings to start from the points.
        let mut window_begin_indices: Vec<usize> = (0..n).collect();

        let mut tecs = Vec::new();

        while window_begin_indices[0] < n {
            // Compute forward diffs in restricted size window
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

            // Sort and partition the diffs to find MTPs
            utilities::sort(&mut forward_diffs);

            let mut mtps: Vec<MTP<T>> = Vec::new();

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
                mtps.push(MTP { translator: *translator, pattern: point_set.get_pattern(&indices) });
            }

            // Split MTP pattern based on max ioi and find translators for subpatterns.
            for mtp in &mtps {
                let split_patterns = SiatecC::split_pattern_on_ioi_gaps(&(*mtp).pattern, self.max_ioi);

                for split_pattern in &split_patterns {
                    let mut translators: Vec<T>;
                    if split_pattern.len() > 1 {
                        translators = SiatecC::find_translators(split_pattern, diff_index, point_set);
                    } else {
                        translators = Vec::new();
                        let pattern_point = split_pattern[0];
                        for p in point_set {
                            if *p != pattern_point {
                                translators.push(*p - pattern_point);
                            }
                        }
                    }

                    tecs.push(TEC { pattern: split_pattern.clone(), translators });
                }
            }
        }

        tecs
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
        // TODO: Reimplement with binary search
        for entry in diff_index {
            if entry.0 == *translation {
                return &entry.1;
            }
        }

        &diff_index[0].1
    }

    fn find_translators<T: Point>(pattern: &Pattern<T>, diff_index: &Vec<(T, Vec<(usize, usize)>)>, point_set: &PointSet<T>) -> Vec<T> {
        let vectorized = pattern.vectorize();
        let v = &vectorized[0];

        let indices = SiatecC::find_indices(diff_index, v);
        let mut a = Vec::with_capacity(indices.len());
        for i in 0..indices.len() {
            a.push(indices[i].1);
        }

        for i in 1..vectorized.len() {
            let v = &vectorized[i];
            let l = SiatecC::find_indices(diff_index, v);
            let mut a_prime = Vec::new();
            let mut j = 0;
            let mut k = 0;

            while j < a.len() && k < l.len() {
                if a[j] == l[k].0 {
                    a_prime.push(l[k].1);
                    j += 1;
                    k += 1;
                } else if a[j] < l[k].0 {
                    j += 1;
                } else if a[j] > l[k].0 {
                    k += 1;
                }
            }

            a = a_prime;
        }

        let mut translators = Vec::new();
        let p = pattern[pattern.len() - 1];
        for i in 0..a.len() {
            let translator = point_set[a[i]] - p;
            if !translator.is_zero() {
                translators.push(translator);
            }
        }

        translators
    }

    pub fn remove_translational_duplicates<T: Point>(tecs: &mut Vec<TEC<T>>) {
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
    use crate::point_set::tec::TEC;
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
        assert_eq!(TEC {
            pattern: Pattern::new(&vec![&a]),
            translators: vec![Point2dF { x: 1.0, y: 0.0 },
                              Point2dF { x: 2.0, y: 0.0 },
                              Point2dF { x: 3.0, y: 0.0 }],
        }, tecs[0]);
        assert_eq!(TEC {
            pattern: Pattern::new(&vec![&a, &b]),
            translators: vec![Point2dF { x: 1.0, y: 0.0 },
                              Point2dF { x: 2.0, y: 0.0 }],
        }, tecs[1]);
        assert_eq!(TEC {
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
        assert_eq!(TEC {
            pattern: Pattern::new(&vec![&a]),
            translators: vec![Point2dF { x: 1.0, y: 0.0 },
                              Point2dF { x: 4.0, y: 0.0 },
                              Point2dF { x: 5.0, y: 0.0 }],
        }, tecs[0]);
        assert_eq!(TEC {
            pattern: Pattern::new(&vec![&a, &b]),
            translators: vec![Point2dF { x: 4.0, y: 0.0 }],
        }, tecs[1]);
    }
}
