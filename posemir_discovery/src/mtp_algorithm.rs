/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use crate::point_set::mtp::Mtp;
use crate::point_set::point::Point;
use crate::point_set::point_set::PointSet;

/// Trait that defines an algorithm that computes MTPs from a point set.
pub trait MtpAlgorithm<T: Point> {
    /// Returns the MTPs in the given point set. Whether all MTPs are returned
    /// depends on the algorithm.
    ///
    /// # Arguments
    ///
    /// * `point_set` - the set of points for which MTPs are computed
    fn compute_mtps(&self, point_set: &PointSet<T>) -> Vec<Mtp<T>>;
}
