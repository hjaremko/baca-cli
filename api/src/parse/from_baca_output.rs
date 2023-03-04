use crate::network::ConnectionConfig;

pub trait FromBacaOutput {
    fn from_baca_output(connection_config: &ConnectionConfig, data: &str) -> Self;
}
