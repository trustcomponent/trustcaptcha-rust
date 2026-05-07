use std::time::Duration;
use trustcaptcha::errors::{
    ApiKeyInvalidError, ServerUnreachableError, VerificationNotFinishedError,
    VerificationNotFoundError, VerificationResultExpiredError,
    VerificationResultRetrievalLimitReachedError, VerificationTokenInvalidError,
};
use trustcaptcha::trust_captcha::TrustCaptcha;

const VALID_TOKEN: &str =
    "eyJ2ZXJpZmljYXRpb25JZCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMCJ9";
const NOT_FOUND_TOKEN: &str =
    "eyJ2ZXJpZmljYXRpb25JZCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMSJ9";
const LOCKED_TOKEN: &str =
    "eyJ2ZXJpZmljYXRpb25JZCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMiJ9";
const EXPIRED_TOKEN: &str =
    "eyJ2ZXJpZmljYXRpb25JZCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMyJ9";
const LIMIT_REACHED_TOKEN: &str =
    "eyJ2ZXJpZmljYXRpb25JZCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwNCJ9";
const TOKEN_WITH_UNKNOWN_FIELDS: &str = "eyJ2ZXJpZmljYXRpb25JZCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMCIsInVua25vd25GaWVsZCI6ImZvbyIsImFub3RoZXJKdW5rIjo0MiwibmVzdGVkIjp7IngiOjF9fQ==";

const VALID_API_KEY: &str = "ak_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";

fn tc(api_key: &str) -> trustcaptcha::trust_captcha::TrustCaptcha {
    TrustCaptcha::builder(api_key).build().expect("builder")
}

#[tokio::test]
async fn test_successful_verification() {
    let result = tc(VALID_API_KEY).get_verification_result(VALID_TOKEN).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_verification_token_invalid() {
    let result = tc(VALID_API_KEY)
        .get_verification_result("invalid_token")
        .await;
    assert!(matches!(
        result
            .unwrap_err()
            .downcast_ref::<VerificationTokenInvalidError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_verification_token_invalid_when_base64_but_not_json() {
    // base64("not-a-json")
    let result = tc(VALID_API_KEY)
        .get_verification_result("bm90LWEtanNvbg==")
        .await;
    assert!(matches!(
        result
            .unwrap_err()
            .downcast_ref::<VerificationTokenInvalidError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_verification_token_invalid_when_json_missing_verification_id() {
    // base64('{"foo":"bar"}')
    let result = tc(VALID_API_KEY)
        .get_verification_result("eyJmb28iOiJiYXIifQ==")
        .await;
    assert!(matches!(
        result
            .unwrap_err()
            .downcast_ref::<VerificationTokenInvalidError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_verification_not_found() {
    let result = tc(VALID_API_KEY)
        .get_verification_result(NOT_FOUND_TOKEN)
        .await;
    assert!(result.is_err());
    assert!(matches!(
        result
            .unwrap_err()
            .downcast_ref::<VerificationNotFoundError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_api_key_invalid() {
    let result = tc("invalid-key").get_verification_result(VALID_TOKEN).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err().downcast_ref::<ApiKeyInvalidError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_verification_not_finished() {
    let result = tc(VALID_API_KEY)
        .get_verification_result(LOCKED_TOKEN)
        .await;
    assert!(result.is_err());
    assert!(matches!(
        result
            .unwrap_err()
            .downcast_ref::<VerificationNotFinishedError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_verification_result_expired() {
    let result = tc(VALID_API_KEY)
        .get_verification_result(EXPIRED_TOKEN)
        .await;
    assert!(result.is_err());
    assert!(matches!(
        result
            .unwrap_err()
            .downcast_ref::<VerificationResultExpiredError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_verification_result_retrieval_limit_reached() {
    let result = tc(VALID_API_KEY)
        .get_verification_result(LIMIT_REACHED_TOKEN)
        .await;
    assert!(result.is_err());
    assert!(matches!(
        result
            .unwrap_err()
            .downcast_ref::<VerificationResultRetrievalLimitReachedError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_tolerates_unknown_fields_in_verification_token() {
    let result = tc(VALID_API_KEY)
        .get_verification_result(TOKEN_WITH_UNKNOWN_FIELDS)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_throws_server_unreachable_on_unreachable_host() {
    let tc = TrustCaptcha::builder(VALID_API_KEY)
        .api_host("http://localhost:1")
        .connect_timeout(Duration::from_millis(500))
        .read_timeout(Duration::from_millis(500))
        .build()
        .expect("builder");
    let result = tc.get_verification_result(VALID_TOKEN).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err().downcast_ref::<ServerUnreachableError>(),
        Some(_)
    ));
}
