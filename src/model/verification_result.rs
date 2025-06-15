use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationResult {
    #[serde(rename = "captchaId")]
    pub captcha_id: Uuid,

    #[serde(rename = "verificationId")]
    pub verification_id: Uuid,

    pub score: f64,

    pub reason: String,

    pub mode: String,

    pub origin: String,

    #[serde(rename = "ipAddress")]
    pub ip_address: String,

    #[serde(rename = "deviceFamily")]
    pub device_family: String,

    #[serde(rename = "operatingSystem")]
    pub operating_system: String,

    pub browser: String,

    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,

    #[serde(rename = "releaseTimestamp")]
    pub release_timestamp: String,

    #[serde(rename = "retrievalTimestamp")]
    pub retrieval_timestamp: String,

    #[serde(rename = "verificationPassed")]
    pub verification_passed: bool,
}
