use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

use crate::error::{from_base64_error, Error};

pub type JsResult<T> = Result<T, Error>;

pub fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    URL_SAFE_NO_PAD.encode(input)
}

pub fn base64_decode<T: AsRef<[u8]>>(context: &'static str, input: T) -> JsResult<Vec<u8>> {
    URL_SAFE_NO_PAD
        .decode(input)
        .map_err(from_base64_error(context))
}
