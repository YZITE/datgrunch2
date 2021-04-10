use crate::InnerMessage;
pub(crate) use sodiumoxide::crypto::sign::ed25519 as crypto;

use crypto::{sign_detached, verify_detached};
pub use crypto::{PublicKey, SecretKey, Signature};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct OuterMessage {
    inner: Vec<u8>,
    signs: BTreeMap<PublicKey, Signature>,
}

impl OuterMessage {
    pub fn try_new(inner: &InnerMessage) -> Result<Self, serde_cbor::Error> {
        serde_cbor::to_vec(inner).map(|inner| OuterMessage {
            inner,
            signs: BTreeMap::new(),
        })
    }

    pub fn attach_signature(&mut self, pk: &PublicKey, sk: &SecretKey) {
        let sig = sign_detached(&self.inner, sk);
        // verify that the secret key and the public key match
        assert!(verify_detached(&sig, &self.inner, pk));
        self.signs.insert(*pk, sig);
    }

    pub fn inner(&self) -> Result<InnerMessage, serde_cbor::Error> {
        serde_cbor::from_slice(&self.inner)
    }

    pub fn verify(&self) -> impl Iterator<Item = (PublicKey, bool)> + '_ {
        self.signs
            .iter()
            .map(move |(pk, sg)| (*pk, verify_detached(sg, &self.inner, pk)))
    }
}
