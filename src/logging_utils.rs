use crate::util;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(get_log_level())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

// todo: generic get env
fn get_log_level() -> Level {
    const VAR_NAME: &str = "BACA_LOG";
    let default_level = "error".to_string();

    // todo: get from command line arguments
    to_log_level(util::get_env(VAR_NAME).unwrap_or(default_level).as_str())
}

fn to_log_level(raw_level: &str) -> Level {
    match raw_level {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "error" => Level::ERROR,
        _ => Level::ERROR,
    }
}
