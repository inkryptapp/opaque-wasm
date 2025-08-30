use opaque_ke::ciphersuite::CipherSuite;

use crate::ksf::CustomKsf;

pub(crate) struct DefaultCipherSuite;

impl CipherSuite for DefaultCipherSuite {
    type OprfCs = opaque_ke::Ristretto255;
    type KeGroup = opaque_ke::Ristretto255;
    type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDh;
    type Ksf = CustomKsf;
}
