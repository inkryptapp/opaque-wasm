use base64::DecodeError;
use opaque_ke::errors::{InternalError, ProtocolError};

pub enum Error {
    Protocol {
        context: &'static str,
        error: ProtocolError,
    },
    Base64 {
        context: &'static str,
        error: DecodeError,
    },
    Internal {
        context: &'static str,
        error: InternalError,
    },
}

pub fn from_base64_error(context: &'static str) -> impl Fn(DecodeError) -> Error {
    move |error| Error::Base64 { context, error }
}

pub fn from_protocol_error(context: &'static str) -> impl Fn(ProtocolError) -> Error {
    move |error| Error::Protocol { context, error }
}
