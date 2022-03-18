use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point;
use crate::point_set::point_set::PointSet;
use crate::point_set::tec::Tec;

#[derive(Debug)]
pub struct TecStats<T: Point> {
    pub tec: Tec<T>,
    pub comp_ratio: f64,
    pub compactness: f64,
    pub covered_set: PointSet<T>,
    pub pattern_width: f64,
    pub pattern_area: f64,
}

pub fn stats_of<T: Point>(tec: Tec<T>, point_set: &PointSet<T>) -> TecStats<T> {
    let covered_set = tec.covered_set();
    let comp_ratio = compr_ratio_with_cov(&tec, &covered_set);
    let bb = bounding_box(&tec.pattern);
    let compactness = bb_compactness(&tec, point_set);

    let pattern_width = bb.upper_x - bb.lower_x;
    let pattern_area = (bb.upper_x - bb.lower_x) * (bb.upper_y - bb.lower_y);

    TecStats {
        tec,
        comp_ratio,
        compactness,
        covered_set,
        pattern_width,
        pattern_area,
    }
}

impl<T: Point> TecStats<T> {
    pub fn is_better_than(&self, other: &TecStats<T>) -> bool {
        if self.comp_ratio > other.comp_ratio {
            return true;
        }
        if self.compactness > other.compactness {
            return true;
        }
        if self.covered_set.len() > other.covered_set.len() {
            return true;
        }
        if self.tec.pattern.len() > other.tec.pattern.len() {
            return true;
        }
        if self.pattern_width < other.pattern_width {
            return true;
        }
        if self.pattern_area < other.pattern_area {
            return true;
        }

        false
    }
}

struct BoundingBox {
    lower_x: f64,
    lower_y: f64,
    upper_x: f64,
    upper_y: f64,
}

impl BoundingBox {
    fn contains<T: Point>(&self, point: &T) -> bool {
        let x = point.component_f64(0).unwrap();
        let y = point.component_f64(1).unwrap();

        if x < self.lower_x {
            return false;
        }

        if x > self.upper_x {
            return false;
        }

        if y < self.lower_y {
            return false;
        }

        if y > self.upper_y {
            return false;
        }

        true
    }
}

fn bounding_box<T: Point>(pattern: &Pattern<T>) -> BoundingBox {
    let mut bb = BoundingBox {
        lower_x: f64::MAX,
        lower_y: f64::MAX,
        upper_x: f64::MIN,
        upper_y: f64::MIN,
    };

    for point in pattern {
        let point_x = point.component_f64(0).unwrap();
        let point_y = point.component_f64(1).unwrap();

        if point_x < bb.lower_x {
            bb.lower_x = point_x;
        }
        if point_x > bb.upper_x {
            bb.upper_x = point_x;
        }
        if point_y < bb.lower_y {
            bb.lower_y = point_y;
        }
        if point_y < bb.upper_y {
            bb.upper_y = point_y;
        }
    }

    bb
}

fn compr_ratio_with_cov<T: Point>(tec: &Tec<T>, cov: &PointSet<T>) -> f64 {
    let cov_size = cov.len() as f64;
    let pat_size = tec.pattern.len() as f64;
    let transl_size = tec.translators.len() as f64;

    // The TEC type is expected to not contain a zero-translator,
    // therefore the denominator does not include the -1 as in [Meredith2013].
    cov_size / (pat_size + transl_size)
}

fn bb_compactness<T: Point>(tec: &Tec<T>, point_set: &PointSet<T>) -> f64 {
    let mut best_compactness = 0.0;
    let expanded = tec.expand();

    for pattern in &expanded {
        let bb = bounding_box(pattern);
        let mut contained: f64 = 0.0;

        for point in point_set {
            if bb.contains(point) {
                contained += 1.0;
            }
        }

        let pat_size = tec.pattern.len() as f64;

        let compactness = pat_size / contained;
        if compactness > best_compactness {
            best_compactness = compactness;
        }
    }

    best_compactness
}
