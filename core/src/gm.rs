use crate::OuterMessage;
pub(crate) use sodiumoxide::crypto::secretbox::xsalsa20poly1305 as crypto;

pub use crypto::{gen_key, Key as GroupKey, Nonce};
use crypto::{gen_nonce, open, seal, NONCEBYTES};
use std::path::Path;
use std::io;

pub const SUBDIR_NAME: &str = "g";

pub fn encode_message(
    m: &OuterMessage,
    gk: &GroupKey,
) -> Result<(Nonce, Vec<u8>), serde_cbor::Error> {
    let data_ue = serde_cbor::to_vec(m)?;
    let nonce = gen_nonce();
    let data = seal(&data_ue, &nonce, gk);
    Ok((nonce, data))
}

pub fn decode_message<'s>(
    nonce: Nonce,
    data: &[u8],
    store: &'s crate::Store,
) -> Result<(&'s str, OuterMessage), serde_cbor::Error> {
    use rayon::prelude::*;

    let (gname, dcdata) = store
        .groupkeys
        .par_iter()
        .map(|(name, key)| open(&data, &nonce, key).map(|dat| (name.as_str(), dat)))
        .find_any(|x| x.is_ok())
        .map_or_else(|| Err(()), |i| i)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "no matching key found"))?;

    let dcdata = serde_cbor::from_slice(&dcdata)?;

    Ok((gname, dcdata))
}

pub fn render_message(
    base_path: &Path,
    m: &OuterMessage,
    gk: &GroupKey,
) -> Result<(), serde_cbor::Error> {
    let (nonce, data) = encode_message(m, gk)?;
    crate::create_message(base_path, SUBDIR_NAME, |f| {
        use io::Write;
        f.write_all(&nonce.0)?;
        f.write_all(&data)?;
        Ok(())
    })?;
    Ok(())
}

pub fn read_message<'s>(
    path: &Path,
    store: &'s crate::Store,
) -> Result<(&'s str, OuterMessage), serde_cbor::Error> {
    let mut nonce = std::fs::read(path)?;
    if nonce.len() < NONCEBYTES {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "not enough data").into());
    }
    let data = nonce.split_off(NONCEBYTES);
    let nonce = Nonce::from_slice(&nonce).unwrap();

    decode_message(nonce, &data, store)
}
