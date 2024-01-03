pub mod utils;

mod components;

use clap::crate_authors;
use crate::utils::exceptions::{initialize_panic_handler, initialize_logging};
use crate::components::calculator::Divisible;

#[tokio::main]
async fn main() {
    initialize_logging()?;

    initialize_panic_handler()?;

    println!("Hello World!");

    println!("Is 3 divisible by 3? {}", components::calculator::is_divisible_x(3, 3));
    println!("Is 3 divisible by 3? {}", 3.divisible(3));
}


