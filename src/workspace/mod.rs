use colored::Colorize;
use std::fs;
use std::fs::DirBuilder;

mod instance_data;
mod zip;

pub use self::instance_data::InstanceData;
pub use self::zip::zip_file;

mod task;
pub use self::task::TaskConfig;

// todo: walk up dir tree until found
const BACA_DIR: &str = ".baca";
const INSTANCE_PATH: &str = ".baca/instance";
const TASK_PATH: &str = ".baca/task";

pub fn initialize() -> Result<(), String> {
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

pub fn save(instance: &InstanceData) {
    let serialized = serde_json::to_string(instance).unwrap();
    tracing::debug!("serialized = {}", serialized);

    fs::write(INSTANCE_PATH, serialized).expect("Unable to write file");
}

// todo: uniform error reporting
pub fn read() -> InstanceData {
    let serialized = fs::read_to_string(INSTANCE_PATH).expect("Unable to read file");
    tracing::debug!("serialized = {}", serialized);

    let deserialized: InstanceData = serde_json::from_str(&serialized).unwrap();
    tracing::debug!("deserialized = {:?}", deserialized);

    deserialized
}

pub fn read_task() -> Option<TaskConfig> {
    tracing::info!("Reading task from workspace.");

    let serialized = fs::read_to_string(TASK_PATH).ok()?;
    tracing::debug!("serialized = {}", serialized);

    let deserialized: TaskConfig = serde_json::from_str(&serialized).ok()?;
    tracing::debug!("deserialized = {:?}", deserialized);

    tracing::info!("Read task successfully.");
    Some(deserialized)
}

pub fn save_task(task_id: &str, filepath: &str, to_zip: bool) {
    tracing::info!("Saving task info to {}.", TASK_PATH);

    let task = TaskConfig {
        id: task_id.to_string(),
        file: filepath.to_string(),
        to_zip,
    };
    let serialized = serde_json::to_string(&task).unwrap();
    tracing::debug!("serialized = {}", serialized);

    fs::write(TASK_PATH, serialized).expect("Unable to write task.");
    tracing::info!("Saved task successfully.");
}

pub fn remove_task() {
    tracing::info!("Removing task from {}.", TASK_PATH);
    match fs::remove_file(TASK_PATH) {
        Ok(_) => {
            tracing::info!("Removed successfully.");
        }
        Err(e) => {
            println!("{}{}", "Error removing task info: ".bright_red(), e);
        }
    }
}
