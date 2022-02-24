use crate::vault::{Secret, SecretAttributes, SmallBuffer};
use crate::Result;
use crate::{async_trait, compat::boxed::Box};

/// A trait for hashing input data into a fixed length output.
#[async_trait]
pub trait Hasher {
    /// Compute the SHA-256 digest given input `data`.
    async fn sha256(&mut self, data: &[u8]) -> Result<[u8; 32]>;
    /// Derive multiple output [`Secret`]s with given attributes using
    /// the HKDF-SHA256 given the specified salt, info and input key
    /// material.
    async fn hkdf_sha256(
        &mut self,
        salt: &Secret,
        info: &[u8],
        ikm: Option<&Secret>,
        output_attributes: SmallBuffer<SecretAttributes>,
    ) -> Result<SmallBuffer<Secret>>;
}
