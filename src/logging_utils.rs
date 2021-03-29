use std::env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(get_log_level())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn get_log_level() -> Level {
    const VAR_NAME: &str = "BACA_LOG";

    // todo: get from command line arguments
    match env::var(VAR_NAME) {
        Ok(raw_level) => to_log_level(raw_level.as_str()),
        Err(_) => {
            tracing::debug!(
                "Variable {} is not present in the environment! Default log level to error.",
                VAR_NAME
            );
            Level::ERROR
        }
    }
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
