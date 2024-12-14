use log::*;

fn main() {
    env_logger::init();
    warn!("one"); // to log a warning
    error!("two"); // to log an error
    info!("three"); // to log an information
    debug!("four");
}

// RUST_LOG=debug cargo run
// [dependencies]
// env_logger = "0.11.5"
// log = "0.4.22"
