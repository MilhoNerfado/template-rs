mod components;
pub mod utils;

use color_eyre::eyre::Result;

use crate::components::calculator::Divisible;
use crate::utils::exceptions::{initialize_logging, initialize_panic_handler};

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logging()?;

    initialize_panic_handler()?;

    println!("Hello World!");

    println!(
        "Is 3 divisible by 3? {}",
        components::calculator::is_divisible_x(3, 3)
    );
    println!("Is 3 divisible by 3? {}", 3.divisible(3));

    Ok(())
}
