use crate::errors::{
    ApiKeyInvalidError, ClientReportedServerUnreachableError, ServerUnreachableError, UnknownError,
    VerificationNotFinishedError, VerificationNotFoundError, VerificationResultExpiredError,
    VerificationResultRetrievalLimitReachedError, VerificationTokenInvalidError,
};
use crate::model::verification_result::VerificationResult;
use crate::model::verification_token::VerificationToken;
use base64::{engine::general_purpose, Engine as _};
use reqwest::{redirect::Policy, Client, Proxy};
use serde_json::json;
use std::error::Error;
use std::time::Duration;

const LIBRARY_VERSION: &str = "3.0.0";
const LIBRARY_LANGUAGE: &str = "rust";
const DEFAULT_API_HOST: &str = "https://api.trustcomponent.com";
const DEFAULT_CONNECT_TIMEOUT_S: u64 = 3;
const DEFAULT_READ_TIMEOUT_S: u64 = 5;

pub struct TrustCaptcha {
    api_key: String,
    api_host: String,
    connect_timeout: Duration,
    read_timeout: Duration,
    proxy: Option<String>,
}

pub struct TrustCaptchaBuilder {
    api_key: String,
    api_host: String,
    connect_timeout: Duration,
    read_timeout: Duration,
    proxy: Option<String>,
}

impl TrustCaptchaBuilder {
    pub fn api_host(mut self, api_host: impl Into<String>) -> Self {
        self.api_host = api_host.into();
        self
    }

    pub fn connect_timeout(mut self, d: Duration) -> Self {
        self.connect_timeout = d;
        self
    }

    pub fn read_timeout(mut self, d: Duration) -> Self {
        self.read_timeout = d;
        self
    }

    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        self.proxy = Some(proxy_url.into());
        self
    }

    pub fn build(self) -> Result<TrustCaptcha, Box<dyn Error>> {
        if self.api_key.is_empty() {
            return Err("api_key must not be empty".into());
        }
        Ok(TrustCaptcha {
            api_key: self.api_key,
            api_host: self.api_host,
            connect_timeout: self.connect_timeout,
            read_timeout: self.read_timeout,
            proxy: self.proxy,
        })
    }
}

impl TrustCaptcha {
    pub fn builder(api_key: impl Into<String>) -> TrustCaptchaBuilder {
        TrustCaptchaBuilder {
            api_key: api_key.into(),
            api_host: DEFAULT_API_HOST.to_string(),
            connect_timeout: Duration::from_secs(DEFAULT_CONNECT_TIMEOUT_S),
            read_timeout: Duration::from_secs(DEFAULT_READ_TIMEOUT_S),
            proxy: None,
        }
    }

    pub async fn get_verification_result(
        &self,
        base64_verification_token: &str,
    ) -> Result<VerificationResult, Box<dyn Error>> {
        let verification_token = match Self::parse_verification_token(base64_verification_token) {
            Ok(t) => t,
            Err(_) => return Err(Box::new(VerificationTokenInvalidError)),
        };

        let url = if verification_token.client_failover {
            format!(
                "{}/v2/verifications/{}/results?clientFailover=true",
                self.api_host, verification_token.verification_id
            )
        } else {
            format!(
                "{}/v2/verifications/{}/results",
                self.api_host, verification_token.verification_id
            )
        };

        let mut builder = Client::builder()
            .redirect(Policy::none())
            .connect_timeout(self.connect_timeout)
            .timeout(self.read_timeout);

        if let Some(proxy_url) = &self.proxy {
            builder = builder.proxy(Proxy::all(proxy_url)?);
        }

        let client = builder.build()?;

        let response = match client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("User-Agent", build_user_agent())
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) if e.is_connect() || e.is_timeout() || e.is_request() => {
                return Err(Box::new(ServerUnreachableError));
            }
            Err(e) => return Err(Box::new(e)),
        };

        match response.status().as_u16() {
            200 => Ok(response.json::<VerificationResult>().await?),
            403 => Err(Box::new(ApiKeyInvalidError)),
            404 => Err(Box::new(VerificationNotFoundError)),
            410 => Err(Box::new(VerificationResultExpiredError)),
            412 => Err(Box::new(ClientReportedServerUnreachableError)),
            423 => Err(Box::new(VerificationNotFinishedError)),
            429 => Err(Box::new(VerificationResultRetrievalLimitReachedError)),
            _ => Err(Box::new(UnknownError)),
        }
    }

    fn parse_verification_token(
        base64_verification_token: &str,
    ) -> Result<VerificationToken, Box<dyn Error>> {
        let decoded = general_purpose::STANDARD.decode(base64_verification_token)?;
        let s = String::from_utf8(decoded)?;
        let token: VerificationToken = serde_json::from_str(&s)?;
        Ok(token)
    }
}

fn build_user_agent() -> String {
    let payload = json!({
        "language": LIBRARY_LANGUAGE,
        "version": LIBRARY_VERSION,
    });
    let encoded = general_purpose::STANDARD.encode(payload.to_string().as_bytes());
    format!("Trustcaptcha/{}", encoded)
}
