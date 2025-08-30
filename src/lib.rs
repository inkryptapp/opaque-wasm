pub mod client;
pub mod server;

mod base64;
mod cipher_suite;
mod error;
mod identifiers;
mod ksf;
mod utils;

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::client::*;
    use crate::server::*;

    #[test]
    fn key_exchange() {
        // Server configuration
        let server_setup = create_server_setup();

        // Client configuration
        let user_identifier = "john.doe@example.com";
        let password = "_P4ssw0rd123!";

        // Registration
        let registration_record = {
            // Client starts registration
            let client_reg_result = start_client_registration(StartClientRegistrationParams {
                password: password.to_string(),
            })
            .unwrap();

            // Server handles registration request
            let server_reg_result =
                create_server_registration_response(CreateServerRegistrationResponseParams {
                    server_setup: server_setup.clone(),
                    user_identifier: user_identifier.to_string(),
                    registration_request: client_reg_result.registration_request,
                })
                .unwrap();

            // Client finishes registration
            let client_finish_result = finish_client_registration(FinishClientRegistrationParams {
                password: password.to_string(),
                registration_response: server_reg_result.registration_response,
                client_registration_state: client_reg_result.client_registration_state,
                identifiers: None,
                key_stretching_function_config: None,
            })
            .unwrap();

            client_finish_result.registration_record
        };

        // Login
        {
            // Client starts login
            let client_login_result = start_client_login(StartClientLoginParams {
                password: password.to_string(),
            })
            .unwrap();

            // Server handles login request
            let server_login_result = start_server_login(StartServerLoginParams {
                server_setup: server_setup.clone(),
                registration_record: Some(registration_record),
                start_login_request: client_login_result.start_login_request,
                user_identifier: user_identifier.to_string(),
                identifiers: None,
            })
            .unwrap();

            // Client finishes login
            let client_finish_result = finish_client_login(FinishClientLoginParams {
                client_login_state: client_login_result.client_login_state,
                login_response: server_login_result.login_response,
                password: password.to_string(),
                identifiers: None,
                key_stretching_function_config: None,
            })
            .unwrap();

            let client_finish_result = client_finish_result.expect("Client login should succeed");

            // Server finishes login
            let server_finish_result = finish_server_login(FinishServerLoginParams {
                server_login_state: server_login_result.server_login_state,
                finish_login_request: client_finish_result.finish_login_request,
            })
            .unwrap();

            // Verify session keys match
            assert_eq!(
                client_finish_result.session_key, server_finish_result.session_key,
                "Session keys should match"
            );
        }
    }
}
