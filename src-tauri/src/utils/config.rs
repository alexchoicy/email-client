use std::sync::Mutex;

use tauri::{Manager, Runtime, State};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    has_account: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config { has_account: false }
    }
}

impl Config {
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        let config_dir = app_handle
            .path()
            .app_config_dir()
            .expect("Failed to get app config directory");

        if !config_dir.exists() {
            std::fs::create_dir(&config_dir).expect("Failed to create config directory");
        }

        let config_path = &config_dir.join("config.json");

        if !config_path.exists() {
            let default_config = Config::default();

            let _ = std::fs::write(
                &config_path,
                serde_json::to_string(&default_config).expect("Failed to serialize default config"),
            );
        }

        let config_data =
            std::fs::read_to_string(&config_path).expect("Failed to read config file");

        let config: Config =
            serde_json::from_str(&config_data).expect("Failed to deserialize config data");

        config
    }

    pub fn set_has_account(&mut self, app_handle: &tauri::AppHandle) {
        self.has_account = true;
        self.save(app_handle);
    }

    pub fn save(&self, app_handle: &tauri::AppHandle) {
        let config_dir = app_handle
            .path()
            .app_config_dir()
            .expect("Failed to get app config directory");

        if !config_dir.exists() {
            std::fs::create_dir(&config_dir).expect("Failed to create config directory");
        }

        let config_path = config_dir.join("config.json");
        let config_data = serde_json::to_string(self).expect("Failed to serialize config");
        std::fs::write(config_path, config_data).expect("Failed to write config file");
    }
}

#[tauri::command]
pub async fn get_config<R: Runtime>(
    app: tauri::AppHandle<R>,
    config: State<'_, Mutex<Config>>,
) -> Result<String, String> {
    let config = config.lock().map_err(|e| e.to_string())?;
    let config_data = serde_json::to_string(&*config).map_err(|e| e.to_string())?;
    Ok(config_data)
}
