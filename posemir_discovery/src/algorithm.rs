/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use crate::point_set::mtp::Mtp;
use crate::point_set::point::Point;
use crate::point_set::point_set::PointSet;
use crate::point_set::tec::Tec;

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

/// Trait for algorithms that compute TECs in a point set.
pub trait TecAlgorithm<T: Point> {
    /// Returns the TECs in the given point set.
    /// The patterns for which the TECs are returned depends on the algorithm.
    ///
    /// # Arguments
    ///
    /// * `point_set` - the set of points for which TECs are computed
    fn compute_tecs(&self, point_set: &PointSet<T>) -> Vec<Tec<T>>;
}
