use base64::DecodeError;
use opaque_ke::errors::{InternalError, ProtocolError};
use wasm_bindgen::prelude::*;

pub(crate) enum Error {
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

pub(crate) fn from_base64_error(context: &'static str) -> impl Fn(DecodeError) -> Error {
    move |error| Error::Base64 { context, error }
}

pub(crate) fn from_protocol_error(context: &'static str) -> impl Fn(ProtocolError) -> Error {
    move |error| Error::Protocol { context, error }
}

impl From<Error> for JsError {
    fn from(err: Error) -> Self {
        let msg = match err {
            Error::Protocol { context, error } => {
                format!("Opaque protocol error at \"{}\"; {}", context, error)
            }
            Error::Base64 { context, error } => {
                format!("base64 decoding failed at \"{}\"; {}", context, error)
            }
            Error::Internal { context, error } => {
                format!("Internal error at \"{}\"; {}", context, error)
            }
        };
        JsError::new(&msg)
    }
}
