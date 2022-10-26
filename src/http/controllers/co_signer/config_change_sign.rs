use std::sync::Arc;

use fireblocks_sdk::{CoSignerCallbackResponse, decrypt_request, CoSignerCallbackResponseAction, encrypt_request};
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use crate::AppContext;

use super::contracts::*;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/v2/config_change_sign_request",
    description: "Configuration Approval Callback Handler",
    controller: "CoSigner",
    input_data: "FireblocksRequest",
    result:[
        {status_code: 200, description: "Ok response", model: "String"},
    ]
)]

pub struct ConfigChangeSignAction {
    app: Arc<AppContext>,
}

impl ConfigChangeSignAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &ConfigChangeSignAction,
    input_data: FireblocksRequest,
    _: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    
    let body_string = String::from_utf8(input_data.body).unwrap();
    println!("Request: {}", body_string);

    let request = decrypt_request(&body_string, action.app.public_key.clone()).unwrap();

    let response = CoSignerCallbackResponse {
        action: CoSignerCallbackResponseAction::Approve,
        request_id: request.request_id,
        reject_reason: "".to_string(),
    };

    let response = encrypt_request(response, action.app.private_key.clone()).unwrap();
    return HttpOutput::as_text(response).into_ok_result(true).into();
}
