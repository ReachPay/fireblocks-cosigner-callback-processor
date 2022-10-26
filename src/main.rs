use std::sync::Arc;

use fireblocks_cosigner_callback_processor::{SettingsReader, APP_NAME, APP_VERSION, AppContext, setup_server, 
    };

use my_seq_logger::SeqLogger;

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".fireblocks-cosigner-callback-processor").await;
    let settings_reader = Arc::new(settings_reader);

    SeqLogger::enable_from_connection_string(
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
        settings_reader.clone(),
        None,
    );

    let app = AppContext::new(&settings_reader).await;
    let app = Arc::new(app);

    
    
    setup_server(app.clone(), 8000);
    app.app_states.wait_until_shutdown().await;
}
