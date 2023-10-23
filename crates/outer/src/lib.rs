use std::sync::Arc;

use chrono::Utc;

uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
pub struct OuterClient();

#[uniffi::export]
impl OuterClient {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self())
    }

    pub async fn get_time(&self) -> chrono::DateTime<Utc> {
        inner::InnerClient::new().get_time()
    }
}

type DateTime = chrono::DateTime<chrono::Utc>;

uniffi::ffi_converter_forward!(DateTime, inner::UniFfiTag, crate::UniFfiTag);
