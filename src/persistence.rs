use crate::baca::InstanceData;
use std::fs;
use std::fs::DirBuilder;
use std::rc::Rc;

const BACA_DIR: &str = ".baca";
const INSTANCE_PATH: &str = ".baca/instance";

pub fn init_repository() -> Result<(), String> {
    let baca_dir = fs::read_dir(BACA_DIR);

    if baca_dir.is_ok() {
        return Err("BaCa directory already exists.".to_string());
    }

    if let Err(e) = DirBuilder::new().create(BACA_DIR) {
        return Err(format!("Error creating BaCa directory: {}", e.to_string()));
    }

    tracing::info!("BaCa directory created successfully.");
    Ok(())
}

pub fn save_baca_info(instance: &InstanceData) {
    let serialized = serde_json::to_string(instance).unwrap();
    tracing::debug!("serialized = {}", serialized);

    fs::write(INSTANCE_PATH, serialized).expect("Unable to write file");
}

// todo: uniform error reporting
pub fn read_baca_info() -> InstanceData {
    let serialized = fs::read_to_string(INSTANCE_PATH).expect("Unable to read file");
    tracing::debug!("serialized = {}", serialized);

    let deserialized: InstanceData = serde_json::from_str(&serialized).unwrap();
    tracing::debug!("deserialized = {:?}", deserialized);

    deserialized
}
