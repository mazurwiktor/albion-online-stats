use core::fmt::{self, Display};

#[derive(Debug)]
pub struct PhotonDecodeError {
    cause: String,
}

impl PhotonDecodeError {
    pub fn extend(self, cause: String) -> Self {
        Self {
            cause: format!("{} {}", cause, self.cause),
        }
    }
}

pub type PhotonDecodeResult<T> = std::result::Result<T, PhotonDecodeError>;

impl Display for PhotonDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.cause, f)
    }
}

// Allows writing `PhotonDecodeError::from("oops"))?`
impl From<&'static str> for PhotonDecodeError {
    fn from(msg: &'static str) -> PhotonDecodeError {
        PhotonDecodeError { cause: msg.into() }
    }
}

// Allows writing `PhotonDecodeError::from("oops".into()))?`
impl From<String> for PhotonDecodeError {
    fn from(msg: String) -> PhotonDecodeError {
        PhotonDecodeError { cause: msg }
    }
}
