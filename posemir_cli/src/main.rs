use clap::{App, Arg};

use crate::application::PoSeMirRunner;

mod application;

pub fn main() {
    let app = App::new("posemir_cli")
        .version("0.1")
        .about("Runs a Point Set Music Information Retrieval algorithm on given input")
        .author("Otso Björklund");

    let app = define_args(app);
    let matches = app.get_matches();

    let mut runner = PoSeMirRunner::new(&matches);
    runner.run();
}

fn define_args(app: App) -> App {
    let app = app.arg(Arg::new("algorithm")
        .long("algo")
        .short('a')
        .takes_value(true)
        .help("The algorithm to run [SIATEC, SIATEC-C, SIA, SIAR]")
        .required(true));

    let app = app.arg(Arg::new("piece")
        .long("piece")
        .short('p')
        .takes_value(true)
        .help("The name of the piece of music")
        .required(true));

    let app = app.arg(Arg::new("input")
        .long("input")
        .short('i')
        .takes_value(true)
        .help("Path (absolute) to the input .csv file")
        .required(true));

    let app = app.arg(Arg::new("output")
        .long("output")
        .short('o')
        .takes_value(true)
        .help("Path (absolute) to the output directory where the output JSON files are written. \
                  For profiling purposes this can be set to /dev/null to avoid file writing operations.")
        .required(true));

    let app = app.arg(Arg::new("batch-size")
        .long("batch-size")
        .short('b')
        .takes_value(true)
        .help("Batch size for output files (= how many patters are written to same output file)")
        .required(false)
        .default_value("100"));

    let app = app.arg(Arg::new("max-ioi")
        .long("max-ioi")
        .takes_value(true)
        .help("Maximum inter-onset interval to use (applies only to SIATEC-C)")
        .required(false)
        .default_value("10.0"));

    let app = app.arg(Arg::new("sub-diagonals")
        .long("sub-diag")
        .takes_value(true)
        .help("Number of subdiagonals to use (applies only to SIAR)")
        .required(false)
        .default_value("3"));

    app
}
