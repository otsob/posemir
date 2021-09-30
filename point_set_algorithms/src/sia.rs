/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
/// Implements the SIA algorithm [Meredith et al. 2002].
/// The SIA algorithm computes all Maximal Translatable Patterns (MTP) in a
/// given point set.
pub mod sia {
    use crate::point_set::mtp::MTP;
    use crate::point_set::point::Point2d;
    use crate::point_set::point_set::PointSet;

    /// Computes and returns all MTPs in the given point set.
    ///
    /// # Arguments
    ///
    /// * `point_set` - The point set for which all MTPS are computed
    pub fn compute_mtps(point_set: &PointSet) -> Vec<MTP> {

        // Compute all differences (translations) between points in the set
        // and store the indices of the points from which the translations are computed.
        let n = point_set.len();
        let mut diffs: Vec<(Point2d, usize)> = Vec::with_capacity(n * (n - 1) / 2);

        for i in 0..n - 1 {
            let from = point_set[i];
            for j in i + 1..n {
                let to = point_set[j];
                diffs.push((to - from, i));
            }
        }

        // Sort all differences
        diffs.sort_by(|a, b| { a.0.cmp(&b.0) });

        // Find MTPs by iterating through the diffs. All points that are translatable
        // by the same translator in the point set are in adjacent positions in the sorted
        // diffs vector.
        let mut mtps: Vec<MTP> = Vec::new();

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
            mtps.push(MTP { translator: *translator, pattern: point_set.get_pattern(&indices) });
        }

        mtps
    }
}

#[cfg(test)]
mod tests {
    use crate::point_set::mtp::MTP;
    use crate::point_set::pattern::Pattern;
    use crate::point_set::point::Point2d;
    use crate::point_set::point_set::PointSet;
    use crate::sia::sia::compute_mtps;

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
        let mut mtps = compute_mtps(&point_set);
        mtps.sort_by(|a, b| { a.translator.cmp(&b.translator) });

        assert_eq!(3, mtps.len());
        assert_eq!(mtps[0], MTP { translator: Point2d { x: 1.0, y: 0.0 }, pattern: Pattern::new(&vec![&a, &b, &c]) });
        assert_eq!(mtps[1], MTP { translator: Point2d { x: 2.0, y: 0.0 }, pattern: Pattern::new(&vec![&a, &b]) });
        assert_eq!(mtps[2], MTP { translator: Point2d { x: 3.0, y: 0.0 }, pattern: Pattern::new(&vec![&a]) });
    }

    #[test]
    fn test_maximal_number_of_mtps() {
        // Create point set where all MTPs only have one point (maximize number of MTPs).
        let mut points = Vec::new();
        let a = Point2d { x: 1.0, y: 1.0 };
        points.push(a);
        let b = Point2d { x: 2.0, y: 2.0 };
        points.push(b);
        let c = Point2d { x: 3.0, y: 4.0 };
        points.push(c);

        let point_set = PointSet::new(points);
        let mut mtps = compute_mtps(&point_set);
        mtps.sort_by(|a, b| { a.translator.cmp(&b.translator) });

        assert_eq!(3, mtps.len());
        assert_eq!(mtps[0], MTP { translator: Point2d { x: 1.0, y: 1.0 }, pattern: Pattern::new(&vec![&a]) });
        assert_eq!(mtps[1], MTP { translator: Point2d { x: 1.0, y: 2.0 }, pattern: Pattern::new(&vec![&b]) });
        assert_eq!(mtps[2], MTP { translator: Point2d { x: 2.0, y: 3.0 }, pattern: Pattern::new(&vec![&a]) });
    }
}
