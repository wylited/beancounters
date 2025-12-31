use std::path::PathBuf;
use std::sync::Mutex;

pub struct AppState {
    pub data_dir: PathBuf,
    // We might want a cache or lock here if we were doing more complex things,
    // but for now we'll just read/write files directly.
    // A mutex might be needed if we want to ensure sequential writes.
    pub write_lock: Mutex<()>,
}

impl AppState {
    pub fn new(data_dir: String) -> anyhow::Result<Self> {
        let path = PathBuf::from(data_dir);
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        Ok(Self {
            data_dir: path,
            write_lock: Mutex::new(()),
        })
    }
}
