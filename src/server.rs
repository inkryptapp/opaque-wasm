use opaque_ke::rand::rngs::OsRng;
use opaque_ke::{
    CredentialFinalization, CredentialRequest, RegistrationRequest, ServerLogin,
    ServerLoginStartParameters, ServerRegistration, ServerSetup,
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::base64::JsResult;
use crate::{
    base64::{base64_decode, base64_encode},
    cipher_suite::DefaultCipherSuite,
    error::from_protocol_error,
    identifiers::{get_identifiers, CustomIdentifiers},
};

#[wasm_bindgen(js_name = createServerSetup)]
pub fn create_server_setup() -> String {
    let mut rng: OsRng = OsRng;
    let setup = ServerSetup::<DefaultCipherSuite>::new(&mut rng);
    base64_encode(setup.serialize())
}

fn decode_server_setup(data: String) -> JsResult<ServerSetup<DefaultCipherSuite>> {
    base64_decode("serverSetup", data).and_then(|bytes| {
        ServerSetup::<DefaultCipherSuite>::deserialize(&bytes)
            .map_err(from_protocol_error("deserialize serverSetup"))
    })
}

#[wasm_bindgen(js_name = getServerPublicKey)]
pub fn get_server_public_key(data: String) -> Result<String, JsError> {
    let server_setup = decode_server_setup(data)?;
    let pub_key = server_setup.keypair().public().serialize();
    Ok(base64_encode(pub_key))
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CreateServerRegistrationResponseParams {
    #[serde(rename = "serverSetup")]
    pub(crate) server_setup: String,
    #[serde(rename = "userIdentifier")]
    pub(crate) user_identifier: String,
    #[serde(rename = "registrationRequest")]
    pub(crate) registration_request: String,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CreateServerRegistrationResponseResult {
    #[serde(rename = "registrationResponse")]
    pub registration_response: String,
}

#[wasm_bindgen(js_name = createServerRegistrationResponse)]
pub fn create_server_registration_response(
    params: CreateServerRegistrationResponseParams,
) -> Result<CreateServerRegistrationResponseResult, JsError> {
    let server_setup = decode_server_setup(params.server_setup)?;
    let registration_request_bytes =
        base64_decode("registrationRequest", params.registration_request)?;
    let server_registration_start_result = ServerRegistration::<DefaultCipherSuite>::start(
        &server_setup,
        RegistrationRequest::deserialize(&registration_request_bytes)
            .map_err(from_protocol_error("deserialize registrationRequest"))?,
        params.user_identifier.as_bytes(),
    )
    .map_err(from_protocol_error("start server registration"))?;
    let registration_response_bytes = server_registration_start_result.message.serialize();

    Ok(CreateServerRegistrationResponseResult {
        registration_response: base64_encode(registration_response_bytes),
    })
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StartServerLoginParams {
    #[serde(rename = "serverSetup")]
    pub(crate) server_setup: String,
    #[serde(rename = "registrationRecord")]
    #[tsify(type = "string | null | undefined")]
    pub(crate) registration_record: Option<String>,
    #[serde(rename = "startLoginRequest")]
    pub(crate) start_login_request: String,
    #[serde(rename = "userIdentifier")]
    pub(crate) user_identifier: String,
    #[tsify(optional)]
    pub(crate) identifiers: Option<CustomIdentifiers>,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StartServerLoginResult {
    #[serde(rename = "serverLoginState")]
    pub(crate) server_login_state: String,
    #[serde(rename = "loginResponse")]
    pub(crate) login_response: String,
}

#[wasm_bindgen(js_name = startServerLogin)]
pub fn start_server_login(
    params: StartServerLoginParams,
) -> Result<StartServerLoginResult, JsError> {
    let server_setup = decode_server_setup(params.server_setup)?;
    let registration_record_bytes = match params.registration_record {
        Some(pw) => base64_decode("registrationRecord", pw).map(Some),
        None => Ok(None),
    }?;
    let credential_request_bytes = base64_decode("startLoginRequest", params.start_login_request)?;

    let mut rng: OsRng = OsRng;

    let registration_record = match registration_record_bytes.as_ref() {
        Some(bytes) => Some(
            ServerRegistration::<DefaultCipherSuite>::deserialize(bytes)
                .map_err(from_protocol_error("deserialize registrationRecord"))?,
        ),
        None => None,
    };

    let start_params = ServerLoginStartParameters {
        identifiers: get_identifiers(&params.identifiers),
        context: None,
    };

    let server_login_start_result = ServerLogin::start(
        &mut rng,
        &server_setup,
        registration_record,
        CredentialRequest::deserialize(&credential_request_bytes)
            .map_err(from_protocol_error("deserialize startLoginRequest"))?,
        params.user_identifier.as_bytes(),
        start_params,
    )
    .map_err(from_protocol_error("start server login"))?;

    let login_response = base64_encode(server_login_start_result.message.serialize());
    let server_login_state = base64_encode(server_login_start_result.state.serialize());

    let result = StartServerLoginResult {
        server_login_state,
        login_response,
    };
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FinishServerLoginParams {
    #[serde(rename = "serverLoginState")]
    pub(crate) server_login_state: String,
    #[serde(rename = "finishLoginRequest")]
    pub(crate) finish_login_request: String,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FinishServerLoginResult {
    #[serde(rename = "sessionKey")]
    pub(crate) session_key: String,
}

#[wasm_bindgen(js_name = finishServerLogin)]
pub fn finish_server_login(
    params: FinishServerLoginParams,
) -> Result<FinishServerLoginResult, JsError> {
    let credential_finalization_bytes =
        base64_decode("finishLoginRequest", params.finish_login_request)?;
    let state_bytes = base64_decode("serverLoginState", params.server_login_state)?;
    let state = ServerLogin::<DefaultCipherSuite>::deserialize(&state_bytes)
        .map_err(from_protocol_error("deserialize serverLoginState"))?;
    let server_login_finish_result = state
        .finish(
            CredentialFinalization::deserialize(&credential_finalization_bytes)
                .map_err(from_protocol_error("deserialize finishLoginRequest"))?,
        )
        .map_err(from_protocol_error("finish server login"))?;
    Ok(FinishServerLoginResult {
        session_key: base64_encode(server_login_finish_result.session_key),
    })
}
