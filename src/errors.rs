use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct VerificationTokenInvalidError;

impl fmt::Display for VerificationTokenInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The verification token is malformed or could not be parsed."
        )
    }
}

impl Error for VerificationTokenInvalidError {}

#[derive(Debug)]
pub struct VerificationNotFoundError;

impl fmt::Display for VerificationNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "No verification could be found for the given verification token."
        )
    }
}

impl Error for VerificationNotFoundError {}

#[derive(Debug)]
pub struct ApiKeyInvalidError;

impl fmt::Display for ApiKeyInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The provided api key is invalid. Please verify the api key from your captcha settings.")
    }
}

impl Error for ApiKeyInvalidError {}

#[derive(Debug)]
pub struct VerificationNotFinishedError;

impl fmt::Display for VerificationNotFinishedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The verification is not yet completed. Please wait until the user has finished solving the captcha before requesting the result.")
    }
}

impl Error for VerificationNotFinishedError {}

#[derive(Debug)]
pub struct VerificationResultExpiredError;

impl fmt::Display for VerificationResultExpiredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The verification result has expired and can no longer be retrieved."
        )
    }
}

impl Error for VerificationResultExpiredError {}

#[derive(Debug)]
pub struct VerificationResultRetrievalLimitReachedError;

impl fmt::Display for VerificationResultRetrievalLimitReachedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The verification result has reached its maximum retrieval count and can no longer be retrieved.")
    }
}

impl Error for VerificationResultRetrievalLimitReachedError {}

#[derive(Debug)]
pub struct UnknownError;

impl fmt::Display for UnknownError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An unknown error occurred")
    }
}

impl Error for UnknownError {}

#[derive(Debug)]
pub struct ServerUnreachableError;

impl fmt::Display for ServerUnreachableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not reach the TrustCaptcha server. This is a high-trust failover signal — your backend was unable to contact our servers.")
    }
}

impl Error for ServerUnreachableError {}

#[derive(Debug)]
pub struct ClientReportedServerUnreachableError;

impl fmt::Display for ClientReportedServerUnreachableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The client reported it could not reach the TrustCaptcha server, but the gateway has no record of a recent outage.")
    }
}

impl Error for ClientReportedServerUnreachableError {}
