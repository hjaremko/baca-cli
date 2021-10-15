use crate::workspace::InstanceData;

pub trait FromBacaOutput {
    fn from_baca_output(instance: &InstanceData, data: &str) -> Self;
}
