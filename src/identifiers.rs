use opaque_ke::Identifiers;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize, Tsify)]
pub struct CustomIdentifiers {
    #[tsify(optional)]
    pub(crate) client: Option<String>,
    #[tsify(optional)]
    pub(crate) server: Option<String>,
}

pub fn get_identifiers(identifiers: &Option<CustomIdentifiers>) -> Identifiers {
    Identifiers {
        client: identifiers
            .as_ref()
            .and_then(|identifiers| identifiers.client.as_ref().map(|val| val.as_bytes())),
        server: identifiers
            .as_ref()
            .and_then(|identifiers| identifiers.server.as_ref().map(|val| val.as_bytes())),
    }
}
