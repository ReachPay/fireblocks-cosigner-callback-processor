use my_http_server_swagger::{MyHttpInput};

#[derive(MyHttpInput)]
pub struct FireblocksRequest {
    #[http_body(description = "Fireblocks data")]
    pub body: Vec<u8>,
}