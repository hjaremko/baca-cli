use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn init_logging(level: Level) {
    let subscriber = FmtSubscriber::builder().without_time().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::debug!("Log level: {}", level);
}
