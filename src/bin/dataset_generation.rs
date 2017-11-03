extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate tetris20g_ai;

use structopt::StructOpt;

use tetris20g_ai::dataset_generator::generate_dataset;

#[derive(StructOpt, Debug)]
#[structopt(name = "dataset_generation", about = "Generate dataset from log file.")]
struct Opt {
    #[structopt(long = "input", help = "Input log file name. (e.g., `dataset/20171101-000000.txt`)")]
    input: String,

    #[structopt(long = "output", default_value = "output.bin",
    help = " Output binary file name.")]
    output: String,

    #[structopt(long = "drop-rate", default_value = "0",
    help = "Rate for drop. This should be between 0.0 and 1.0.")]
    drop_rate: f64,

    #[structopt(long = "weights-file",
    help = "If not specified, drop is uniformly done for all candidates at random. Otherwise, \
weights file is loaded and bottom `drop_rate` of candidates in terms of value scores are dropped.")]
    weights_file: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    generate_dataset(&opt.input, &opt.output, opt.drop_rate, opt.weights_file);
}
