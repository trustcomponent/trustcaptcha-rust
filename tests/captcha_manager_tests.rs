use trustcaptcha::captcha_manager::CaptchaManager;
use trustcaptcha::errors::{
    SecretKeyInvalidError, VerificationNotFinishedError, VerificationNotFoundError,
    VerificationTokenInvalidError,
};

const VALID_TOKEN: &str = "eyJhcGlFbmRwb2ludCI6Imh0dHBzOi8vYXBpLnRydXN0Y29tcG9uZW50LmNvbSIsInZlcmlmaWNhdGlvbklkIjoiMDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAwIiwiZW5jcnlwdGVkQWNjZXNzVG9rZW4iOiJ0b2tlbiJ9";
const NOT_FOUND_TOKEN: &str = "eyJhcGlFbmRwb2ludCI6Imh0dHBzOi8vYXBpLnRydXN0Y29tcG9uZW50LmNvbSIsInZlcmlmaWNhdGlvbklkIjoiMDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAxIiwiZW5jcnlwdGVkQWNjZXNzVG9rZW4iOiJ0b2tlbiJ9";
const LOCKED_TOKEN: &str = "eyJhcGlFbmRwb2ludCI6Imh0dHBzOi8vYXBpLnRydXN0Y29tcG9uZW50LmNvbSIsInZlcmlmaWNhdGlvbklkIjoiMDAwMDAwMDAtMDAwMC0wMDAwLTAwMDAtMDAwMDAwMDAwMDAyIiwiZW5jcnlwdGVkQWNjZXNzVG9rZW4iOiJ0b2tlbiJ9";

#[tokio::test]
async fn test_successful_verification() {
    let result = CaptchaManager::get_verification_result("secret-key", VALID_TOKEN).await;
    println!("Result: {:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_verification_token_invalid() {
    let result = CaptchaManager::get_verification_result("secret-key", "invalid_token").await;
    println!("Result: {:?}", result);
    assert!(matches!(
        result
            .unwrap_err()
            .downcast_ref::<VerificationTokenInvalidError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_verification_not_found() {
    let result = CaptchaManager::get_verification_result("secret-key", NOT_FOUND_TOKEN).await;
    assert!(result.is_err());
    assert!(matches!(
        result
            .unwrap_err()
            .downcast_ref::<VerificationNotFoundError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_secret_key_invalid() {
    let result = CaptchaManager::get_verification_result("invalid-key", VALID_TOKEN).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err().downcast_ref::<SecretKeyInvalidError>(),
        Some(_)
    ));
}

#[tokio::test]
async fn test_verification_not_finished() {
    let result = CaptchaManager::get_verification_result("secret-key", LOCKED_TOKEN).await;
    assert!(result.is_err());
    assert!(matches!(
        result
            .unwrap_err()
            .downcast_ref::<VerificationNotFinishedError>(),
        Some(_)
    ));
}
