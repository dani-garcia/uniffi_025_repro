use std::sync::Arc;

use chrono::Utc;

uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
pub struct InnerClient();

#[uniffi::export]
impl InnerClient {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self())
    }

    pub fn get_time(&self) -> chrono::DateTime<Utc> {
        Utc::now()
    }
}

type DateTime = chrono::DateTime<chrono::Utc>;
uniffi::custom_type!(DateTime, std::time::SystemTime);

impl UniffiCustomTypeConverter for chrono::DateTime<chrono::Utc> {
    type Builtin = std::time::SystemTime;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Self::from(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.into()
    }
}
