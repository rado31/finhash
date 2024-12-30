pub mod cli;
pub mod utils;

pub fn run() {
    let args = cli::Params::new();
    let hashes = utils::create_hashes(args);

    for hash in hashes {
        println!("{hash}");
    }
}
