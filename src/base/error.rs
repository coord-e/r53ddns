use std::{error, fmt, net, result};

use rusoto_core::RusotoError;
use rusoto_route53 as rusoto;

#[derive(Debug)]
pub enum Error {
    GetIP(reqwest::Error),
    GetChange(rusoto::GetChangeError),
    ChangeResourceRecordSets(rusoto::ChangeResourceRecordSetsError),
    TLSClient(rusoto_core::request::TlsError),
    Credential(rusoto_credential::CredentialsError),
    HttpDispatch(rusoto_core::request::HttpDispatchError),
    InvalidIPAddress(net::AddrParseError),
    UnknownChangeStatus(String),
    Blocking,
    AWS(String),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::GetIP(e) => write!(f, "Failed to obtain current global IP: {}", e),
            Error::GetChange(e) => write!(f, "'GetChange' failed: {}", e),
            Error::ChangeResourceRecordSets(e) => {
                write!(f, "'ChangeResourceRecordSets' failed: {}", e)
            }
            Error::TLSClient(e) => write!(f, "Failed to create TLS client: {}", e),
            Error::Credential(e) => write!(f, "Invalid credential: {}", e),
            Error::HttpDispatch(e) => write!(f, "{}", e),
            Error::InvalidIPAddress(e) => write!(f, "Invalid IP address: {}", e),
            Error::UnknownChangeStatus(e) => write!(f, "Unknown change status is returned: {}", e),
            Error::Blocking => write!(f, "Failed to run blocking future"),
            Error::AWS(e) => write!(f, "AWS error: {}", e),
        }
    }
}

impl error::Error for Error {}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Self {
        Error::InvalidIPAddress(err)
    }
}

impl From<rusoto::GetChangeError> for Error {
    fn from(err: rusoto::GetChangeError) -> Self {
        Error::GetChange(err)
    }
}

impl From<rusoto::ChangeResourceRecordSetsError> for Error {
    fn from(err: rusoto::ChangeResourceRecordSetsError) -> Self {
        Error::ChangeResourceRecordSets(err)
    }
}

impl From<rusoto_core::request::TlsError> for Error {
    fn from(err: rusoto_core::request::TlsError) -> Self {
        Error::TLSClient(err)
    }
}

impl<E> From<RusotoError<E>> for Error
where
    E: Into<Error>,
{
    fn from(err: RusotoError<E>) -> Self {
        match err {
            RusotoError::Service(e) => e.into(),
            RusotoError::HttpDispatch(e) => Error::HttpDispatch(e),
            RusotoError::Credentials(e) => Error::Credential(e),
            RusotoError::Validation(e) => Error::AWS(e),
            RusotoError::ParseError(e) => Error::AWS(e),
            RusotoError::Unknown(e) => Error::AWS(e.body_as_str().to_string()),
            RusotoError::Blocking => Error::Blocking,
        }
    }
}
