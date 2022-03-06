/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point;
use crate::point_set::point_set::PointSet;

/// Represents a translational equivalence class (see [Meredith et al. 2002]).
/// A TEC consists of a pattern and all of its translationally equivalent occurrences in a point set.
/// TECs are represented as a pattern and the translators by which it can be translated
/// to produce all of the translationally equivalent occurrences. The translators do *not* contain
/// the zero vector.
#[derive(Debug, Clone)]
pub struct Tec<T: Point> {
    pub pattern: Pattern<T>,
    pub translators: Vec<T>,
}

impl<T: Point> Tec<T> {
    /// Returns the expansion of this TEC.
    ///
    /// The TEC is expanded by creating all translated copies of the pattern.
    pub fn expand(&self) -> Vec<Pattern<T>> {
        let mut occurrences = Vec::with_capacity(self.translators.len() + 1);
        occurrences.push(self.pattern.clone());

        for translator in &self.translators {
            occurrences.push(self.pattern.translate(translator));
        }

        occurrences
    }

    /// Returns the set of points covered by this TEC.
    pub fn covered_set(&self) -> PointSet<T> {
        let expanded = self.expand();
        let mut points = Vec::new();
        for pattern in &expanded {
            for point in pattern {
                points.push(*point);
            }
        }

        PointSet::new(points)
    }
}

impl<T: Point> PartialEq for Tec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.translators == other.translators && self.pattern == other.pattern
    }
}

impl<T: Point> Eq for Tec<T> {}

#[cfg(test)]
mod tests {
    use crate::point_set::pattern::Pattern;
    use crate::point_set::point::Point2Df64;
    use crate::point_set::tec::Tec;

    #[test]
    fn test_covered_set() {
        let pattern = Pattern::new(&vec![
            &Point2Df64 { x: 1.0, y: 0.0 },
            &Point2Df64 { x: 2.0, y: 0.0 },
        ]);
        let translators = vec![Point2Df64 { x: 1.0, y: 0.0 }, Point2Df64 { x: 1.0, y: 1.0 }];
        let tec = Tec {
            pattern,
            translators,
        };

        let cov = tec.covered_set();

        assert_eq!(5, cov.len());
        assert_eq!(Point2Df64 { x: 1.0, y: 0.0 }, cov[0]);
        assert_eq!(Point2Df64 { x: 2.0, y: 0.0 }, cov[1]);
        assert_eq!(Point2Df64 { x: 2.0, y: 1.0 }, cov[2]);
        assert_eq!(Point2Df64 { x: 3.0, y: 0.0 }, cov[3]);
        assert_eq!(Point2Df64 { x: 3.0, y: 1.0 }, cov[4]);
    }
}
