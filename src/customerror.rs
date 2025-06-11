use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    WalletNotFound,
    ExceedsBalance,
    BadRequest,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::WalletNotFound => write!(f, "Not Found"),
            CustomError::ExceedsBalance => write!(f, "Exceeds balance"),
            CustomError::BadRequest => write!(f, "Bad Request")
        }
    }
}

impl std::error::Error for CustomError {}