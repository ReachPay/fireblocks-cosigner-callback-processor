use std::sync::Arc;
use rust_extensions::AppStates;

use crate::{
    SettingsModel,
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: SettingsModel,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>
}

impl AppContext {
    pub async fn new(settings_reader: &Arc<crate::settings::SettingsReader>) -> Self {
        let settings = settings_reader.get_settings().await;
        let app_states = Arc::new(AppStates::create_initialized());

        let public_key = std::fs::read(&"/certs/public.pem").unwrap();
        let private_key = std::fs::read(&"/certs/private.pem").unwrap();

        Self {
            app_states,
            settings,
            public_key: public_key,
            private_key: private_key,
        }
    }
}

