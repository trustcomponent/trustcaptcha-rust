use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationResult {
    #[serde(rename = "captchaId")]
    pub captcha_id: Uuid,

    #[serde(rename = "verificationId")]
    pub verification_id: Uuid,

    #[serde(rename = "verificationPassed")]
    pub verification_passed: bool,

    pub score: f64,

    #[serde(rename = "decisionType")]
    pub decision_type: String,

    #[serde(rename = "decisionAction")]
    pub decision_action: String,

    #[serde(rename = "gatewayFailoverActive")]
    pub gateway_failover_active: bool,

    #[serde(rename = "riskScoringEnabled")]
    pub risk_scoring_enabled: bool,

    #[serde(rename = "minimalDataModeEnabled")]
    pub minimal_data_mode_enabled: bool,

    pub origin: String,

    #[serde(rename = "ipAddress")]
    pub ip_address: String,

    #[serde(rename = "countryCode")]
    pub country_code: String,

    #[serde(rename = "deviceFamily")]
    pub device_family: String,

    #[serde(rename = "operatingSystem")]
    pub operating_system: String,

    pub browser: String,

    #[serde(rename = "verificationStartedAt")]
    pub verification_started_at: String,

    #[serde(rename = "verificationFinishedAt")]
    pub verification_finished_at: String,

    #[serde(rename = "resultExpiresAt")]
    pub result_expires_at: String,

    #[serde(rename = "resultFirstFetchedAt")]
    pub result_first_fetched_at: String,

    #[serde(rename = "resultLastFetchedAt")]
    pub result_last_fetched_at: String,
}
