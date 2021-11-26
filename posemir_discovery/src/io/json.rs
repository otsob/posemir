use std::fs::File;
use std::path::Path;

use serde_json::{json, Value};

use crate::point_set::pattern::Pattern;
use crate::point_set::point::Point;
use crate::point_set::point::Point2Df64;
use crate::point_set::tec::Tec;

/// Write a set of TECs into a JSON file, following the following format for each TEC:
/// ```json
/// {
///    "piece": "Beethoven op.1",
///    "pattern": {
///     "label": "P3",
///     "source": "siatec",
///     "data_type": "point_set",
///     "data": [
///       [
///         1.0,
///         64.0
///       ],
///       [
///         2.0,
///         60.0
///       ]
///            ...
///     ]
///   },
///   "occurrences": [ list of pattern objects ]
/// }
/// ```
///
/// # Arguments:
/// * `piece` - Name of the piece
/// * `source` - The source of the TECs, e.g, algorithm or analysts name.
/// * `tecs` - The TECs that are written to JSON
/// * `path` - Output path
pub fn write_tecs_to_json(piece: &str, source: &str, tecs: &Vec<Tec<Point2Df64>>, path: &Path) {
    let mut json_values = Vec::new();

    for (i, tec) in tecs.iter().enumerate() {
        let label = &format!("P{}", i);
        let expanded = tec.expand();
        let pattern = pattern_to_json(label, source, &expanded[0]);
        let occurrences: Vec<Value> = expanded[1..].iter().map(|p| { pattern_to_json(label, source, p) }).collect();

        json_values.push(json!({
            "piece": piece,
            "pattern": pattern,
            "occurrences": occurrences
        }));
    }

    serde_json::to_writer_pretty(&File::create(path).unwrap(), &json_values).unwrap()
}

fn pattern_to_json(label: &str, source: &str, pattern: &Pattern<Point2Df64>) -> Value {
    let data: Vec<Value> = pattern.into_iter()
        .map(|p| {
            Value::Array(vec![json!(p.component_f64(0).unwrap()),
                              json!(p.component_f64(1).unwrap())])
        })
        .collect();

    json!({
        "label": label,
        "source": source,
        "data_type": "point_set",
        "data": data
    })
}
