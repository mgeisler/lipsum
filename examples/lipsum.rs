extern crate lipsum;

use std::env;

fn main() {
    // First command line argument or "" if not supplied.
    let arg = env::args().nth(1).unwrap_or_default();
    // Number of words to generate.
    let n = arg.parse().unwrap_or(25);
    // Print n words of lorem ipsum text.
    println!("{}", lipsum::lipsum(n));
}
