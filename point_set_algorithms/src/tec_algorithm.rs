/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use crate::point_set::point_set::PointSet;
use crate::point_set::tec::TEC;

/// Trait for algorithms that compute TECs in a point set.
pub trait TecAlgorithm {
    /// Returns the TECs in the given point set.
    /// The patterns for which the TECs are returned depends on the algorithm.
    ///
    /// # Arguments
    ///
    /// * `point_set` - the set of points for which TECs are computed
    fn compute_tecs(&self, point_set: &PointSet) -> Vec<TEC>;
}
