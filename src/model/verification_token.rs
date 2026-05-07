use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationToken {
    #[serde(rename = "verificationId")]
    pub verification_id: Uuid,

    #[serde(rename = "clientFailover", default)]
    pub client_failover: bool,
}
