/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point2d;

/// Represents a Maximal Translatable Pattern (MTP) [Meredith et al. 2002].
/// An MTP is the set of all points in a point set D that can be
/// translated by a vector so that the translated points are also
/// within the point set D.
#[derive(Debug)]
pub struct MTP {
    pub translator: Point2d,
    pub pattern: Pattern<Point2d>,
}

impl PartialEq for MTP {
    fn eq(&self, other: &Self) -> bool {
        self.translator == other.translator && self.pattern == other.pattern
    }
}

impl Eq for MTP {}

