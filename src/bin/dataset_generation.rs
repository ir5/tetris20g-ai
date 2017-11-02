extern crate structopt;
#[macro_use]
extern crate structopt_derive;

extern crate tetris20g_ai;

use structopt::StructOpt;

use tetris20g_ai::dataset_generator::generate_dataset;

#[derive(StructOpt, Debug)]
#[structopt(name = "20G")]
struct Opt {
    #[structopt(long = "input")]
    input: String,

    #[structopt(long = "output", default_value = "output.bin")]
    output: String,

    #[structopt(long = "drop-rate", default_value = "0")]
    drop_rate: f64,

    #[structopt(long = "weights-file")]
    weights_file: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    generate_dataset(&opt.input, &opt.output, opt.drop_rate, opt.weights_file);
}
