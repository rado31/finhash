use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Params {
    /// number of zeros
    #[arg(short = 'N', value_parser = clap::value_parser!(u32).range(1..))]
    pub n: u32,

    /// number of results
    #[arg(short = 'F', value_parser = clap::value_parser!(u32).range(1..))]
    pub f: u32,

    /// number of threads
    #[arg(default_value_t = 5)]
    pub threads: u32,
}

impl Params {
    pub fn new() -> Self {
        Self::parse()
    }
}
