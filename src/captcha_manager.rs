use crate::errors::{
    SecretKeyInvalidError, UnknownError, VerificationNotFinishedError, VerificationNotFoundError,
    VerificationTokenInvalidError,
};
use crate::model::verification_result::VerificationResult;
use crate::model::verification_token::VerificationToken;
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use std::error::Error;

pub struct CaptchaManager;

impl CaptchaManager {
    pub async fn get_verification_result(
        secret_key: &str,
        base64_verification_token: &str,
    ) -> Result<VerificationResult, Box<dyn Error>> {
        let verification_token =
            match CaptchaManager::get_verification_token(base64_verification_token) {
                Ok(token) => token,
                Err(_) => return Err(Box::new(VerificationTokenInvalidError)),
            };
        CaptchaManager::fetch_verification_result(&verification_token, secret_key).await
    }

    fn get_verification_token(
        base64_verification_token: &str,
    ) -> Result<VerificationToken, Box<dyn Error>> {
        let decoded_token = general_purpose::STANDARD.decode(base64_verification_token)?;
        let decoded_str = String::from_utf8(decoded_token)?;
        let verification_token: VerificationToken = serde_json::from_str(&decoded_str)?;
        Ok(verification_token)
    }

    async fn fetch_verification_result(
        verification_token: &VerificationToken,
        secret_key: &str,
    ) -> Result<VerificationResult, Box<dyn Error>> {
        let url = format!(
            "{}/verifications/{}/assessments",
            verification_token.api_endpoint, verification_token.verification_id
        );
        let client = Client::new();
        let response = client
            .get(&url)
            .header("tc-authorization", secret_key)
            .header("tc-library-language", "rust")
            .header("tc-library-version", "2.0")
            .send()
            .await?;

        match response.status().as_u16() {
            200 => {
                let verification_result: VerificationResult = response.json().await?;
                Ok(verification_result)
            }
            403 => Err(Box::new(SecretKeyInvalidError)),
            404 => Err(Box::new(VerificationNotFoundError)),
            423 => Err(Box::new(VerificationNotFinishedError)),
            _ => Err(Box::new(UnknownError)),
        }
    }
}
