use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationToken {
    #[serde(rename = "apiEndpoint")]
    pub api_endpoint: String,

    #[serde(rename = "verificationId")]
    pub verification_id: Uuid,
}
