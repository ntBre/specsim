use std::fs::read_to_string;

use clap::Parser;
use specsim::{LineShape, Spectrum};

#[derive(Parser)]
struct Args {
    /// input file with lines of the form:
    ///
    /// FREQ INTEN
    ///
    /// skipping lines with a leading #, which are interpreted as comments
    input: String,

    /// Line shape for the output
    #[arg(short, long, value_enum, default_value_t = LineShape::Gaussian)]
    line_shape: LineShape,

    #[arg(short, long, default_value_t = 4000)]
    npoints: usize,

    #[arg(short, long, default_value_t = 1.0)]
    deltag: f64,
}

fn main() {
    let args = Args::parse();

    let Ok(s) = read_to_string(&args.input) else {
        eprintln!("failed to read {}", args.input);
        std::process::exit(1);
    };

    let spec = Spectrum::load(s);
    let (x, y) = spec.sim(args.line_shape, args.npoints, args.deltag);

    for (x, y) in x.iter().zip(y) {
        println!("{x:10.4} {y:10.8}");
    }
}
