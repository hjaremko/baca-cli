use crate::workspace::ConnectionConfig;

pub trait FromBacaOutput {
    fn from_baca_output(connection_config: &ConnectionConfig, data: &str) -> Self;
}
