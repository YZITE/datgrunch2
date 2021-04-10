use crate::OuterMessage;
pub(crate) use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305 as crypto;

pub use crypto::{PublicKey, SecretKey};
use sodiumoxide::crypto::sealedbox::curve25519blake2bxsalsa20poly1305::{open, seal};
use std::io;
use std::path::Path;

pub const SUBDIR_NAME: &str = "d";

pub fn encode_message(m: &OuterMessage, dk: &PublicKey) -> Result<Vec<u8>, serde_cbor::Error> {
    Ok(seal(&serde_cbor::to_vec(m)?, dk))
}

pub fn decode_message(
    data: &[u8],
    pk: &PublicKey,
    sk: &SecretKey,
) -> Result<OuterMessage, serde_cbor::Error> {
    let data = open(&data, pk, sk)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "no matching key found"))?;
    serde_cbor::from_slice(&data)
}

pub fn render_message(
    base_path: &Path,
    m: &OuterMessage,
    dk: &PublicKey,
) -> Result<(), serde_cbor::Error> {
    let data = encode_message(m, dk)?;
    crate::create_message(base_path, SUBDIR_NAME, |f| {
        use io::Write;
        f.write_all(&data)?;
        Ok(())
    })?;
    Ok(())
}

pub fn read_message(
    path: &Path,
    pk: &PublicKey,
    sk: &SecretKey,
) -> Result<OuterMessage, serde_cbor::Error> {
    decode_message(&std::fs::read(path)?, pk, sk)
}
