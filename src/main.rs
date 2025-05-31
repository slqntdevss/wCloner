use std::time::Duration;
use colored::Colorize;
use tokio::time::Instant;

mod cloner;

use cloner::clone::clone;

mod utils {
    pub mod requests;
    pub mod errors;
    #[macro_use]
    pub mod log;
}

#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    match (clone("https://www.ixl.com", "/signin").await.expect("uh oh bad cloner ur gonna get touched")) {
        true => {
            log!("wCloner successfully cloned {} in {:.5}s", "https://www.ixl.com", start_time.elapsed().as_secs_f32())
        }
        false => {
            log!("wCloner failed for an unknown reason.");
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(99999)).await;
}
