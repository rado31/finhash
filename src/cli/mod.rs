use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Params {
    /// number of zeros
    #[arg(short = 'N', value_parser = clap::value_parser!(u8).range(1..))]
    pub n: u8,

    /// number of results
    #[arg(short = 'F', value_parser = clap::value_parser!(u8).range(1..))]
    pub f: u8,
}

impl Params {
    pub fn new() -> Self {
        Self::parse()
    }
}
