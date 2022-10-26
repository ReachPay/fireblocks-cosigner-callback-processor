use std::sync::Arc;

use fireblocks_cosigner_callback_processor::{AppContext, setup_server};

#[tokio::main]
async fn main() {
    let app = AppContext::new().await;
    let app = Arc::new(app);
    
    setup_server(app.clone(), 8000);
    app.app_states.wait_until_shutdown().await;
}
