use std::path::PathBuf;

use clap::ArgMatches;

use posemir_discovery::algorithm::{MtpAlgorithm, TecAlgorithm};
use posemir_discovery::io::csv::csv_to_2d_point_f64;
use posemir_discovery::io::json::write_tecs_to_json;
use posemir_discovery::point_set::mtp::Mtp;
use posemir_discovery::point_set::point::Point2Df64;
use posemir_discovery::point_set::point_set::PointSet;
use posemir_discovery::point_set::tec::Tec;
use posemir_discovery::sia::Sia;
use posemir_discovery::siar::SiaR;
use posemir_discovery::siatec::Siatec;
use posemir_discovery::siatec_c::SiatecC;
use posemir_discovery::siatec_ch::SiatecCH;

type Point = Point2Df64;

pub struct PoSeMirRunner {
    input_path: PathBuf,
    output_writer: OutputWriter,
    sub_diag: usize,
    max_ioi: f64,
}

struct OutputWriter {
    algorithm: String,
    piece: String,
    output_dir_path: PathBuf,
    batch: Vec<Tec<Point>>,
    batch_number: usize,
    batch_size: usize,
    output_count: usize,
}

impl OutputWriter {
    pub fn output_mtp(&mut self, mtp: Mtp<Point>) {
        let tec: Tec<Point> = Tec { pattern: mtp.pattern.clone(), translators: vec![mtp.translator] };
        self.output_tec(tec);
    }

    pub fn output_tec(&mut self, tec: Tec<Point>) {
        self.batch.push(tec);

        if self.batch.len() >= self.batch_size {
            self.flush();
        }
    }

    pub fn flush(&mut self) {
        if self.output_dir_path.to_str().unwrap() != "/dev/null" {
            let mut output_path = self.output_dir_path.clone();
            output_path.push(format!("patterns_{}_{}_{}.json", self.piece, self.algorithm, self.batch_number));
            write_tecs_to_json(&self.piece, &self.algorithm, &self.batch, output_path.as_path());
        }

        self.output_count += self.batch.len();
        self.batch.clear();
        self.batch_number += 1;
    }
}


impl PoSeMirRunner {
    pub fn new(matches: &ArgMatches) -> PoSeMirRunner {
        let algorithm = matches.value_of("algorithm").unwrap().to_uppercase();
        let input_path = matches.value_of("input").unwrap();
        let output_path = matches.value_of("output").unwrap();
        let batch_size: usize = matches.value_of("batch-size").unwrap().parse().unwrap();

        let piece = matches.value_of("piece").unwrap();

        let sub_diag: usize = matches.value_of("sub-diagonals").unwrap().parse().unwrap();
        let max_ioi: f64 = matches.value_of("max-ioi").unwrap().parse().unwrap();

        PoSeMirRunner {
            input_path: PathBuf::from(input_path),
            output_writer: OutputWriter {
                algorithm: algorithm.to_string(),
                piece: piece.to_string(),
                output_dir_path: PathBuf::from(output_path),
                batch: Vec::new(),
                batch_number: 0,
                batch_size,
                output_count: 0,
            },
            sub_diag,
            max_ioi,
        }
    }

    pub fn run(&mut self) {
        let input_data = csv_to_2d_point_f64(&self.input_path);
        match input_data {
            Ok(points) => {
                println!("Loaded {:?}, size {} points", &self.output_writer.piece, points.len());
                self.compute_patterns(points);
            }
            Err(error) => {
                println!("Failed to read input file: {}", error);
            }
        }
    }

    fn compute_patterns(&mut self, points: Vec<Point2Df64>) {
        let point_set = PointSet::new(points);

        let mut name = String::from(&self.output_writer.algorithm);
        match name.as_str() {
            "SIA" => {
                Sia {}.compute_mtps_to_output(&point_set, |mtp| { self.output_writer.output_mtp(mtp) });
            }
            "SIAR" => {
                SiaR { r: self.sub_diag }.compute_mtps_to_output(&point_set, |mtp| { self.output_writer.output_mtp(mtp) });
                name.push_str(&format!(" (r={})", self.sub_diag));
            }
            "SIATEC" => {
                Siatec {}.compute_tecs_to_output(&point_set, |tec| { self.output_writer.output_tec(tec) });
            }
            "SIATEC-C" => {
                SiatecC { max_ioi: self.max_ioi }.compute_tecs_to_output(&point_set, |tec| { self.output_writer.output_tec(tec) });
                name.push_str(&format!(" (max-ioi={})", self.max_ioi));
            }
            "SIATEC-CH" => {
                SiatecCH { max_ioi: self.max_ioi }.compute_tecs_to_output(&point_set, |tec| { self.output_writer.output_tec(tec) });
                name.push_str(&format!(" (max-ioi={})", self.max_ioi));
            }
            _ => {
                println!("Unrecognized algorithm: {}", name);
            }
        }

        // Ensure all patterns written to files.
        self.output_writer.flush();
        println!("Executed {} and saved {} patterns.", name, self.output_writer.output_count);
    }
}

