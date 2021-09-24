/*
 * (c) Otso Björklund (2021)
 * Distributed under the MIT license (see LICENSE.txt or https://opensource.org/licenses/MIT).
 */
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;

use csv::StringRecord;

use crate::point_set::point::Point2d;

#[derive(Debug)]
struct MissingValueError(usize);

impl Display for MissingValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value missing at column {}", self.0)
    }
}

impl Error for MissingValueError {}

fn get_value_at(record: &StringRecord, i: usize) -> Result<f64, Box<dyn Error>> {
    let str_opt = record.get(i);

    match str_opt {
        None => Err(Box::new(MissingValueError(i))),
        Some(str) => Ok(str.trim().parse::<f64>()?)
    }
}

/// Returns a vector of points read from the CSV file at the given path.
/// The CSV file is expected to:
/// - have a header row
/// - contain x-coordinates in the first column
/// - contain y-coordinates in the second column
///
/// The rest of the columns are ignored.
///
/// # Arguments
///
/// * `path` - The path to the CSV file
///
pub fn read_csv_to_points(path: &Path) -> Result<Vec<Point2d>, Box<dyn Error>> {
    let mut points = Vec::new();
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;

        let x = get_value_at(&record, 0)?;
        let y = get_value_at(&record, 1)?;

        points.push(Point2d { x, y });
    }

    Ok(points)
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::io::csv::read_csv_to_points;
    use crate::point_set::point::Point2d;

    #[test]
    fn test_csv_to_points() {
        // Create tempfile and put data in it
        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        let content = "x, y \n -1.0, 2.0 \n 0.0, 3.0 \n 2.1, 1.1 \n";
        tmp_file.write_all(content.as_bytes()).unwrap();

        let mut points = read_csv_to_points(tmp_file.path()).unwrap();
        points.sort();

        assert_eq!(Point2d { x: -1.0, y: 2.0 }, points[0]);
        assert_eq!(Point2d { x: 0.0, y: 3.0 }, points[1]);
        assert_eq!(Point2d { x: 2.1, y: 1.1 }, points[2]);
    }
}
