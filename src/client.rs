use opaque_ke::rand::rngs::OsRng;
use opaque_ke::{
    ClientLogin, ClientLoginFinishParameters, ClientRegistration,
    ClientRegistrationFinishParameters, CredentialResponse, RegistrationResponse,
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::{
    base64::{base64_decode, base64_encode},
    cipher_suite::DefaultCipherSuite,
    error::from_protocol_error,
    identifiers::{get_identifiers, CustomIdentifiers},
    ksf::{get_custom_ksf, KeyStretchingFunctionConfig},
};

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StartClientLoginParams {
    pub(crate) password: String,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StartClientLoginResult {
    #[serde(rename = "clientLoginState")]
    pub(crate) client_login_state: String,
    #[serde(rename = "startLoginRequest")]
    pub(crate) start_login_request: String,
}

#[wasm_bindgen(js_name = startClientLogin)]
pub fn start_client_login(
    params: StartClientLoginParams,
) -> Result<StartClientLoginResult, JsError> {
    let mut client_rng = OsRng;
    let client_login_start_result =
        ClientLogin::<DefaultCipherSuite>::start(&mut client_rng, params.password.as_bytes())
            .map_err(from_protocol_error("start client login"))?;

    let result = StartClientLoginResult {
        client_login_state: base64_encode(client_login_start_result.state.serialize()),
        start_login_request: base64_encode(client_login_start_result.message.serialize()),
    };
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FinishClientLoginParams {
    #[serde(rename = "clientLoginState")]
    pub(crate) client_login_state: String,
    #[serde(rename = "loginResponse")]
    pub(crate) login_response: String,
    pub(crate) password: String,
    #[tsify(optional)]
    pub(crate) identifiers: Option<CustomIdentifiers>,
    #[tsify(optional)]
    #[serde(rename = "keyStretching")]
    pub(crate) key_stretching_function_config: Option<KeyStretchingFunctionConfig>,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FinishClientLoginResult {
    #[serde(rename = "finishLoginRequest")]
    pub(crate) finish_login_request: String,
    #[serde(rename = "sessionKey")]
    pub(crate) session_key: String,
    #[serde(rename = "exportKey")]
    pub(crate) export_key: String,
    #[serde(rename = "serverStaticPublicKey")]
    pub(crate) server_static_public_key: String,
}

#[wasm_bindgen(js_name = finishClientLogin)]
pub fn finish_client_login(
    params: FinishClientLoginParams,
) -> Result<Option<FinishClientLoginResult>, JsError> {
    let custom_ksf = get_custom_ksf(params.key_stretching_function_config)?;

    let credential_response_bytes = base64_decode("loginResponse", params.login_response)?;
    let state_bytes = base64_decode("clientLoginState", params.client_login_state)?;
    let state = ClientLogin::<DefaultCipherSuite>::deserialize(&state_bytes)
        .map_err(from_protocol_error("deserialize clientLoginState"))?;

    let finish_params = ClientLoginFinishParameters::new(
        None,
        get_identifiers(&params.identifiers),
        custom_ksf.as_ref(),
    );

    let result = state.finish(
        params.password.as_bytes(),
        CredentialResponse::deserialize(&credential_response_bytes)
            .map_err(from_protocol_error("deserialize loginResponse"))?,
        finish_params,
    );

    if result.is_err() {
        // Client-detected login failure
        return Ok(None);
    }
    let client_login_finish_result = result.unwrap();

    Ok(Some(FinishClientLoginResult {
        finish_login_request: base64_encode(client_login_finish_result.message.serialize()),
        session_key: base64_encode(client_login_finish_result.session_key),
        export_key: base64_encode(client_login_finish_result.export_key),
        server_static_public_key: base64_encode(client_login_finish_result.server_s_pk.serialize()),
    }))
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StartClientRegistrationParams {
    pub(crate) password: String,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StartClientRegistrationResult {
    #[serde(rename = "clientRegistrationState")]
    pub(crate) client_registration_state: String,
    #[serde(rename = "registrationRequest")]
    pub(crate) registration_request: String,
}

#[wasm_bindgen(js_name = startClientRegistration)]
pub fn start_client_registration(
    params: StartClientRegistrationParams,
) -> Result<StartClientRegistrationResult, JsError> {
    let mut client_rng = OsRng;

    let client_registration_start_result = ClientRegistration::<DefaultCipherSuite>::start(
        &mut client_rng,
        params.password.as_bytes(),
    )
    .map_err(from_protocol_error("start client registration"))?;

    let result = StartClientRegistrationResult {
        client_registration_state: base64_encode(
            client_registration_start_result.state.serialize(),
        ),
        registration_request: base64_encode(client_registration_start_result.message.serialize()),
    };
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FinishClientRegistrationParams {
    pub(crate) password: String,
    #[serde(rename = "registrationResponse")]
    pub(crate) registration_response: String,
    #[serde(rename = "clientRegistrationState")]
    pub(crate) client_registration_state: String,
    #[tsify(optional)]
    pub(crate) identifiers: Option<CustomIdentifiers>,
    #[tsify(optional)]
    #[serde(rename = "keyStretching")]
    pub(crate) key_stretching_function_config: Option<KeyStretchingFunctionConfig>,
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FinishClientRegistrationResult {
    #[serde(rename = "registrationRecord")]
    pub(crate) registration_record: String,
    #[serde(rename = "exportKey")]
    pub(crate) export_key: String,
    #[serde(rename = "serverStaticPublicKey")]
    pub(crate) server_static_public_key: String,
}

#[wasm_bindgen(js_name = finishClientRegistration)]
pub fn finish_client_registration(
    params: FinishClientRegistrationParams,
) -> Result<FinishClientRegistrationResult, JsError> {
    let custom_ksf = get_custom_ksf(params.key_stretching_function_config)?;

    let registration_response_bytes =
        base64_decode("registrationResponse", params.registration_response)?;
    let mut rng: OsRng = OsRng;
    let client_registration =
        base64_decode("clientRegistrationState", params.client_registration_state)?;
    let state = ClientRegistration::<DefaultCipherSuite>::deserialize(&client_registration)
        .map_err(from_protocol_error("deserialize clientRegistrationState"))?;

    let finish_params = ClientRegistrationFinishParameters::new(
        get_identifiers(&params.identifiers),
        custom_ksf.as_ref(),
    );

    let client_finish_registration_result = state
        .finish(
            &mut rng,
            params.password.as_bytes(),
            RegistrationResponse::deserialize(&registration_response_bytes)
                .map_err(from_protocol_error("deserialize registrationResponse"))?,
            finish_params,
        )
        .map_err(from_protocol_error("finish client registration"))?;

    let registration_record_bytes = client_finish_registration_result.message.serialize();
    let result = FinishClientRegistrationResult {
        registration_record: base64_encode(registration_record_bytes),
        export_key: base64_encode(client_finish_registration_result.export_key),
        server_static_public_key: base64_encode(
            client_finish_registration_result.server_s_pk.serialize(),
        ),
    };
    Ok(result)
}
