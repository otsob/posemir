/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use crate::point_set::mtp::MTP;
use crate::point_set::point::Point2d;
use crate::point_set::point_set::PointSet;

/// Trait that defines an algorithm that computes MTPs from a point set.
pub trait MtpAlgorithm {
    /// Returns the MTPs in the given point set. Whether all MTPs are returned
    /// depends on the algorithm.
    ///
    /// # Arguments
    ///
    /// * `point_set` - the set of points for which MTPs are computed
    fn compute_mtps(&self, point_set: &PointSet<Point2d>) -> Vec<MTP>;
}
