extern crate cities;
extern crate docopt;

use docopt::Docopt;
use std::process;

const USAGE: &'static str = "
Computes the distance between two cities.

Usage:
    cities <origin> <destination>

Options:
    -h --help     Show this screen.
";

fn main() {
    let args: cities::Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if let Err(e) = cities::run(args) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
