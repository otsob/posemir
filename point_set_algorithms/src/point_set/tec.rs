/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point2d;

/// Represents a translational equivalence class (see [Meredith et al. 2002]).
/// A TEC consists of a pattern and all of its translationally equivalent occurrences in a point set.
/// TECs are represented as a pattern and the translators by which it can be translated
/// to produce all of the translationally equivalent occurrences. The translators do *not* contain
/// the zero vector.
#[derive(Debug)]
pub struct TEC {
    pub pattern: Pattern<Point2d>,
    pub translators: Vec<Point2d>,
}

impl TEC {
    /// Returns the expansion of this TEC.
    ///
    /// The TEC is expanded by creating all translated copies of the pattern.
    pub fn expand(&self) -> Vec<Pattern<Point2d>> {
        let mut occurrences = Vec::with_capacity(self.translators.len() + 1);
        occurrences.push(self.pattern.clone());

        for translator in &self.translators {
            occurrences.push(self.pattern.translate(translator));
        }

        occurrences
    }
}


impl PartialEq for TEC {
    fn eq(&self, other: &Self) -> bool {
        self.translators == other.translators && self.pattern == other.pattern
    }
}

impl Eq for TEC {}
