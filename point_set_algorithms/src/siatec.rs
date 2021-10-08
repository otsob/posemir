/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
/// Implements the SIATEC algorithm for computing all translational equivalence classes (TECs) of
/// maximal translatable patterns (MTPs) in a point set (see [Meredith et al 2002]). The implementation
/// is based on the pseudocode in Figure 13.7 of [Meredith 2016] and on the description in [Meredith et al 2002]
/// that avoids computing TECs for duplicate MTPs.
pub mod siatec {
    use std::cmp::Ordering;

    use crate::point_set::pattern::Pattern;
    use crate::point_set::point::Point2d;
    use crate::point_set::point_set::PointSet;
    use crate::point_set::tec::TEC;

    pub fn compute_tecs(point_set: &PointSet) -> Vec<TEC> {
        let n = point_set.len();

        // Compute full difference vector table and difference vectors forwards
        let mut diff_table = create_diff_table(n);
        let mut forward_diffs: Vec<(Point2d, usize)> = Vec::with_capacity(n * (n - 1) / 2);

        for i in 0..n {
            let from = &point_set[i];

            for j in 0..n {
                let to = &point_set[j];
                let diff = (to - from, i);
                diff_table[i].push(diff);

                if i < j {
                    forward_diffs.push(diff);
                }
            }
        }

        // Sort differences, compute MTPs and store the patterns along with their point indices
        forward_diffs.sort_by(|a, b| { a.0.cmp(&b.0) });
        let mut mtp_indices: Vec<(Pattern, Pattern, Vec<usize>)> = Vec::new();

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
            let pattern = point_set.get_pattern(&indices);
            let vectorized = pattern.vectorize();
            mtp_indices.push((pattern, vectorized, indices));
        }

        // Remove duplicate TECs
        // Sort by the vectorized representations so that translationally
        // equivalent patterns are adjacent.
        mtp_indices.sort_by(|a, b| {
            let size_order = a.1.len().cmp(&b.1.len());
            if size_order == Ordering::Equal {
                return a.1.cmp(&b.1);
            }
            size_order
        });

        // Store only the translationally distinct MTPs
        let mut distinct_mtps = Vec::new();
        let mut vec = &mtp_indices[0].1;
        distinct_mtps.push((&mtp_indices[0].0, &mtp_indices[0].2));
        for mtp_triplet in &mtp_indices {
            if mtp_triplet.1 != *vec {
                distinct_mtps.push((&mtp_triplet.0, &mtp_triplet.2));
                vec = &mtp_triplet.1;
            }
        }

        let mut tecs = Vec::new();

        // Find translators
        for mtp_indices in &distinct_mtps {
            let pattern = mtp_indices.0;
            let pat_len = pattern.len();
            let col_ind = mtp_indices.1;

            let initial_value: usize = 0;
            let mut row_ind = vec![initial_value; pat_len];

            let mut translators: Vec<Point2d> = Vec::new();

            while row_ind[0] <= n - pat_len {
                for j in 1..pat_len {
                    row_ind[j] = row_ind[0] + j;
                }

                let vec = diff_table[col_ind[0]][row_ind[0]].0;
                let mut found = false;

                for c in 1..pat_len {
                    while row_ind[c] < n && diff_table[col_ind[c]][row_ind[c]].0 < vec {
                        row_ind[c] += 1;
                    }

                    if row_ind[c] >= n || vec != diff_table[col_ind[c]][row_ind[c]].0 {
                        break;
                    }

                    if c == pat_len - 1 {
                        found = true;
                    }
                }

                if (found || pat_len == 1) && !vec.is_zero() {
                    translators.push(vec);
                }

                row_ind[0] += 1;
            }

            tecs.push(TEC { pattern: pattern.clone(), translators });
        }

        tecs
    }

    fn create_diff_table(size: usize) -> Vec<Vec<(Point2d, usize)>> {
        let mut diff_table: Vec<Vec<(Point2d, usize)>> = Vec::with_capacity(size);
        for _ in 0..size {
            diff_table.push(Vec::with_capacity(size));
        }

        diff_table
    }
}

#[cfg(test)]
mod tests {
    use crate::point_set::pattern::Pattern;
    use crate::point_set::point::Point2d;
    use crate::point_set::point_set::PointSet;
    use crate::point_set::tec::TEC;
    use crate::siatec::siatec;

    #[test]
    fn test_with_minimal_number_of_mtps() {
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
        let mut tecs = siatec::compute_tecs(&point_set);
        tecs.sort_by(|a, b| { a.pattern.len().cmp(&b.pattern.len()) });

        assert_eq!(3, tecs.len());
        assert_eq!(TEC {
            pattern: Pattern::new(&vec![&a]),
            translators: vec![Point2d { x: 1.0, y: 0.0 },
                              Point2d { x: 2.0, y: 0.0 },
                              Point2d { x: 3.0, y: 0.0 }],
        }, tecs[0]);
        assert_eq!(TEC {
            pattern: Pattern::new(&vec![&a, &b]),
            translators: vec![Point2d { x: 1.0, y: 0.0 },
                              Point2d { x: 2.0, y: 0.0 }],
        }, tecs[1]);
        assert_eq!(TEC {
            pattern: Pattern::new(&vec![&a, &b, &c]),
            translators: vec![Point2d { x: 1.0, y: 0.0 }],
        }, tecs[2]);
    }
}
