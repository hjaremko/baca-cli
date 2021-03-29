mod logging_utils;

fn main() {
    logging_utils::init_logging();

    tracing::info!("Hello world!");
    tracing::warn!("Hello world!");
    tracing::error!("Hello world!");
    tracing::debug!("Hello world!");
}
