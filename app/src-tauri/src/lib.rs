use std::sync::Mutex;
use tauri::State;

use osnova_lib::services::{
    AppsService, BottomMenuTab, ConfigService, IdentityService, KeyService, LauncherService,
    NavigationService, StatusService, Theme, UIService,
};

/// Application state holding all services
pub struct AppState {
    // Services are wrapped in Mutex for interior mutability
    identity_service: Mutex<Option<IdentityService>>,
    key_service: Mutex<Option<KeyService>>,
    config_service: Mutex<Option<ConfigService>>,
    apps_service: Mutex<Option<AppsService>>,
    launcher_service: Mutex<Option<LauncherService>>,
    ui_service: Mutex<Option<UIService>>,
    navigation_service: Mutex<Option<NavigationService>>,
    status_service: Mutex<StatusService>,
    storage_path: String,
}

impl AppState {
    pub fn new(storage_path: String) -> Self {
        Self {
            identity_service: Mutex::new(None),
            key_service: Mutex::new(None),
            config_service: Mutex::new(None),
            apps_service: Mutex::new(None),
            launcher_service: Mutex::new(None),
            ui_service: Mutex::new(None),
            navigation_service: Mutex::new(None),
            status_service: Mutex::new(StatusService::new()),
            storage_path,
        }
    }

    /// Derive cocoon key for key service
    fn derive_cocoon_key(user_id: &str, master_key: &[u8; 32]) -> [u8; 32] {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(b"osnova-key-service-cocoon:");
        hasher.update(user_id.as_bytes());
        hasher.update(master_key);
        let hash = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(hash.as_bytes());
        key
    }

    /// Initialize services for a specific user
    pub fn init_for_user(&self, user_id: &str) -> Result<(), String> {
        // Initialize identity service
        let identity_service =
            IdentityService::new(&self.storage_path).map_err(|e| e.to_string())?;
        *self.identity_service.lock().unwrap() = Some(identity_service);

        // Get identity to derive key service cocoon key
        let identity = self
            .identity_service
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get_identity()
            .map_err(|e| e.to_string())?;

        // Derive cocoon key from identity
        let cocoon_key = Self::derive_cocoon_key(user_id, identity.master_key());

        // Initialize key service
        let key_service =
            KeyService::new(&self.storage_path, &cocoon_key).map_err(|e| e.to_string())?;
        *self.key_service.lock().unwrap() = Some(key_service);

        // Initialize config service
        let config_service = ConfigService::new(&self.storage_path).map_err(|e| e.to_string())?;
        *self.config_service.lock().unwrap() = Some(config_service);

        // Initialize apps service
        let apps_service = AppsService::new(&self.storage_path).map_err(|e| e.to_string())?;
        *self.apps_service.lock().unwrap() = Some(apps_service);

        // Initialize launcher service
        let launcher_service =
            LauncherService::new(&self.storage_path, user_id).map_err(|e| e.to_string())?;
        *self.launcher_service.lock().unwrap() = Some(launcher_service);

        // Initialize UI service
        let ui_service = UIService::new(&self.storage_path, user_id).map_err(|e| e.to_string())?;
        *self.ui_service.lock().unwrap() = Some(ui_service);

        // Initialize navigation service
        let navigation_service =
            NavigationService::new(&self.storage_path, user_id).map_err(|e| e.to_string())?;
        *self.navigation_service.lock().unwrap() = Some(navigation_service);

        Ok(())
    }
}

// ============================================================================
// Identity Service Commands
// ============================================================================

#[tauri::command]
fn identity_create(state: State<AppState>) -> Result<String, String> {
    let guard = state.identity_service.lock().unwrap();
    let service = guard.as_ref().ok_or("Identity service not initialized")?;
    let (seed_phrase, _address) = service.create().map_err(|e| e.to_string())?;
    Ok(seed_phrase)
}

#[tauri::command]
fn identity_import(state: State<AppState>, seed_phrase: String) -> Result<String, String> {
    let guard = state.identity_service.lock().unwrap();
    let service = guard.as_ref().ok_or("Identity service not initialized")?;
    let address = service.import_with_phrase(&seed_phrase).map_err(|e| e.to_string())?;
    Ok(address)
}

#[tauri::command]
fn identity_get(state: State<AppState>) -> Result<String, String> {
    let guard = state.identity_service.lock().unwrap();
    let service = guard.as_ref().ok_or("Identity service not initialized")?;
    let identity = service.get_identity().map_err(|e| e.to_string())?;
    // Return fingerprint as hex string
    let fingerprint = identity.fingerprint();
    Ok(hex::encode(fingerprint))
}

// ============================================================================
// Apps Service Commands
// ============================================================================

#[tauri::command]
fn apps_list(state: State<AppState>) -> Result<String, String> {
    let guard = state.apps_service.lock().unwrap();
    let service = guard.as_ref().ok_or("Apps service not initialized")?;
    let apps = service.list().map_err(|e| e.to_string())?;
    serde_json::to_string(&apps).map_err(|e| e.to_string())
}

#[tauri::command]
fn apps_launch(state: State<AppState>, app_id: String) -> Result<(), String> {
    let guard = state.apps_service.lock().unwrap();
    let service = guard.as_ref().ok_or("Apps service not initialized")?;
    service.launch(&app_id).map_err(|e| e.to_string())
}

// ============================================================================
// Launcher Service Commands
// ============================================================================

#[tauri::command]
fn launcher_get_layout(state: State<AppState>) -> Result<String, String> {
    let guard = state.launcher_service.lock().unwrap();
    let service = guard.as_ref().ok_or("Launcher service not initialized")?;
    let layout = service.get_layout().map_err(|e| e.to_string())?;
    serde_json::to_string(&layout.app_ids).map_err(|e| e.to_string())
}

#[tauri::command]
fn launcher_set_layout(state: State<AppState>, app_ids: Vec<String>) -> Result<(), String> {
    let guard = state.launcher_service.lock().unwrap();
    let service = guard.as_ref().ok_or("Launcher service not initialized")?;
    service.set_layout(app_ids).map_err(|e| e.to_string())
}

// ============================================================================
// UI Service Commands
// ============================================================================

#[tauri::command]
fn ui_get_theme(state: State<AppState>) -> Result<String, String> {
    let guard = state.ui_service.lock().unwrap();
    let service = guard.as_ref().ok_or("UI service not initialized")?;
    let theme = service.get_theme().map_err(|e| e.to_string())?;
    Ok(match theme {
        Theme::Light => "light".to_string(),
        Theme::Dark => "dark".to_string(),
        Theme::System => "system".to_string(),
    })
}

#[tauri::command]
fn ui_set_theme(state: State<AppState>, theme: String) -> Result<(), String> {
    let guard = state.ui_service.lock().unwrap();
    let service = guard.as_ref().ok_or("UI service not initialized")?;
    let theme_enum = match theme.as_str() {
        "light" => Theme::Light,
        "dark" => Theme::Dark,
        "system" => Theme::System,
        _ => return Err("Invalid theme value".to_string()),
    };
    service.set_theme(theme_enum).map_err(|e| e.to_string())
}

// ============================================================================
// Navigation Service Commands
// ============================================================================

#[tauri::command]
fn navigation_get_bottom_menu(state: State<AppState>) -> Result<String, String> {
    let guard = state.navigation_service.lock().unwrap();
    let service = guard.as_ref().ok_or("Navigation service not initialized")?;
    let tab = service.get_bottom_menu().map_err(|e| e.to_string())?;
    Ok(match tab {
        BottomMenuTab::Launcher => "launcher".to_string(),
        BottomMenuTab::Wallet => "wallet".to_string(),
        BottomMenuTab::Config => "config".to_string(),
    })
}

#[tauri::command]
fn navigation_set_bottom_menu(state: State<AppState>, tab: String) -> Result<(), String> {
    let guard = state.navigation_service.lock().unwrap();
    let service = guard.as_ref().ok_or("Navigation service not initialized")?;
    let tab_enum = match tab.as_str() {
        "launcher" => BottomMenuTab::Launcher,
        "wallet" => BottomMenuTab::Wallet,
        "config" => BottomMenuTab::Config,
        _ => return Err("Invalid tab value".to_string()),
    };
    service.set_bottom_menu(tab_enum).map_err(|e| e.to_string())
}

// ============================================================================
// Status Service Commands
// ============================================================================

#[tauri::command]
fn status_get_server(state: State<AppState>) -> Result<String, String> {
    let guard = state.status_service.lock().unwrap();
    let status = guard.get_server().map_err(|e| e.to_string())?;
    serde_json::to_string(&status).map_err(|e| e.to_string())
}

// ============================================================================
// Tauri Entry Point
// ============================================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Determine storage path
    let storage_path = std::env::var("OSNOVA_STORAGE_PATH").unwrap_or_else(|_| {
        let mut path = dirs::data_local_dir().expect("Failed to get local data dir");
        path.push("osnova");
        path.to_str().unwrap().to_string()
    });

    let app_state = AppState::new(storage_path);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            identity_create,
            identity_import,
            identity_get,
            apps_list,
            apps_launch,
            launcher_get_layout,
            launcher_set_layout,
            ui_get_theme,
            ui_set_theme,
            navigation_get_bottom_menu,
            navigation_set_bottom_menu,
            status_get_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
