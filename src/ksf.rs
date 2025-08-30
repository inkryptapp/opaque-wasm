use argon2::{Algorithm, Argon2, ParamsBuilder, Version};
use generic_array::{ArrayLength, GenericArray};
use opaque_ke::{errors::InternalError, ksf::Ksf};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum KeyStretchingFunctionConfig {
    #[serde(rename = "rfc-9106-recommended")]
    Rfc9106Recommended,
    #[serde(rename = "libsodium-moderate")]
    LibsodiumModerate,
    #[serde(rename = "memory-constrained")]
    MemoryConstrained,
    #[serde(rename = "argon2id-custom")]
    Custom {
        #[serde(rename = "iterations")]
        iterations: u32,
        #[serde(rename = "memory")]
        memory: u32,
        #[serde(rename = "parallelism")]
        parallelism: u32,
    },
}

#[derive(Default)]
pub struct CustomKsf {
    argon: Argon2<'static>,
}

/// Used for the key stretching function in OPAQUE
impl Ksf for CustomKsf {
    /// Computes the key stretching function
    fn hash<L: ArrayLength<u8>>(
        &self,
        input: generic_array::GenericArray<u8, L>,
    ) -> Result<GenericArray<u8, L>, InternalError> {
        let mut output = GenericArray::default();
        self.argon
            .hash_password_into(&input, &[0; argon2::RECOMMENDED_SALT_LEN], &mut output)
            .map_err(|_| InternalError::KsfError)?;
        Ok(output)
    }
}

fn build_argon2_ksf(
    t_cost: u32,
    m_cost: u32,
    parallelism: u32,
) -> Result<Option<CustomKsf>, Error> {
    let mut param_builder = ParamsBuilder::default();
    param_builder.t_cost(t_cost);
    param_builder.m_cost(m_cost);
    param_builder.p_cost(parallelism);

    if let Ok(params) = param_builder.build() {
        let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        return Ok(Some(CustomKsf { argon }));
    }

    Err(Error::Internal {
        context: "Invalid keyStretching (argon2id) combination",
        error: InternalError::KsfError,
    })
}

pub fn get_custom_ksf(
    ksf_config: Option<KeyStretchingFunctionConfig>,
) -> Result<Option<CustomKsf>, Error> {
    let config = ksf_config.unwrap_or(KeyStretchingFunctionConfig::LibsodiumModerate);

    match config {
        // https://www.rfc-editor.org/rfc/rfc9106.html#section-4-6.1
        // using the recommended parameters for Argon2id except we use 2^21-1 since 2^21 crashes in browsers
        KeyStretchingFunctionConfig::Rfc9106Recommended => {
            build_argon2_ksf(1, u32::pow(2, 21) - 1, 4)
        }
        // https://www.rfc-editor.org/rfc/rfc9106.html#section-4-6.2
        KeyStretchingFunctionConfig::MemoryConstrained => build_argon2_ksf(3, u32::pow(2, 16), 4),
        // https://libsodium.gitbook.io/doc/password_hashing/default_phf#key-derivation
        KeyStretchingFunctionConfig::LibsodiumModerate => build_argon2_ksf(3, u32::pow(2, 18), 4),
        KeyStretchingFunctionConfig::Custom {
            iterations,
            memory,
            parallelism,
        } => build_argon2_ksf(iterations, memory, parallelism),
    }
}
