use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct VerificationTokenInvalidError;

impl fmt::Display for VerificationTokenInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid verification token")
    }
}

impl Error for VerificationTokenInvalidError {}

#[derive(Debug)]
pub struct VerificationNotFoundError;

impl fmt::Display for VerificationNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Verification not found")
    }
}

impl Error for VerificationNotFoundError {}

#[derive(Debug)]
pub struct SecretKeyInvalidError;

impl fmt::Display for SecretKeyInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Secret key is invalid")
    }
}

impl Error for SecretKeyInvalidError {}

#[derive(Debug)]
pub struct VerificationNotFinishedError;

impl fmt::Display for VerificationNotFinishedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Verification not finished")
    }
}

impl Error for VerificationNotFinishedError {}

#[derive(Debug)]
pub struct UnknownError;

impl fmt::Display for UnknownError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An unknown error occurred")
    }
}

impl Error for UnknownError {}
