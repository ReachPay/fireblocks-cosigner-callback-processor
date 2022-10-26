use std::sync::Arc;

use my_http_server_controllers::controllers::ControllersMiddleware;

use crate::{FireblocksSignTxAction, ConfigChangeSignAction, AppContext};


pub fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new();

    result.register_post_action(Arc::new(
        ConfigChangeSignAction::new(
            app.clone(),
        ),
    ));

    result.register_post_action(Arc::new(
        FireblocksSignTxAction::new(
            app.clone(),
        ),
    ));

    result
}
