use base64::URL_SAFE_NO_PAD as BASE64_CONFIG;
use serde::{Deserialize, Serialize};
use sodiumoxide::randombytes::randombytes_into;
use std::collections::HashMap;
use std::io;
use std::path::Path;

/// Message signatures
pub mod sign;
pub use sign::{OuterMessage, Signature};

/// Message indices
mod listings;
pub use listings::{parse_listings, update_listings};

/// Direct Messages
pub mod dm;

/// Group Messages
pub mod gm;
use gm::GroupKey;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicKey {
    pub msgs: dm::PublicKey,
    pub signs: sign::PublicKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub pubkeys: HashMap<String, PublicKey>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MessagePart {
    Text(String),
    GroupKey(GroupKey),
    Misc(Vec<u8>),
}

pub type InnerMessage = HashMap<String, MessagePart>;

pub fn init() {
    sodiumoxide::init().unwrap();
}

pub fn base_path() -> std::path::PathBuf {
    std::env::var("DATGRUNCH2_DIR")
        .unwrap_or_else(|_| ".".to_string())
        .into()
}

pub fn prepare_base_path(base_path: &Path) -> io::Result<()> {
    fn create_dir2e(path: &Path) -> io::Result<()> {
        std::fs::create_dir(path).or_else(|e| {
            if e.kind() == io::ErrorKind::AlreadyExists {
                Ok(())
            } else {
                Err(e)
            }
        })
    }

    std::fs::create_dir_all(base_path)?;
    create_dir2e(&base_path.join("d"))?;
    create_dir2e(&base_path.join("g"))?;
    Ok(())
}

fn generate_message_id() -> String {
    let mut tmp = Vec::new();
    tmp.extend(&chrono::Utc::now().timestamp().to_be_bytes());
    let tcl = tmp.len();
    tmp.extend_from_slice(&[0u8; 16]);
    randombytes_into(&mut tmp[tcl..]);
    base64::encode_config(tmp, BASE64_CONFIG)
}

fn create_message<F>(base_path: &Path, subdir: &str, mangler: F) -> io::Result<()>
where
    F: FnOnce(&mut std::fs::File) -> io::Result<()>,
{
    let name = base_path.join(subdir).join(generate_message_id());
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(name)?;
    mangler(&mut f)?;
    use io::Write;
    f.flush()?;
    f.sync_all()?;
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Store {
    pub groupkeys: HashMap<String, GroupKey>,
    pub msgkey: dm::crypto::SecretKey,
    pub sgnkey: sign::crypto::SecretKey,
    pub pubkey: PublicKey,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        let (mpk, msgkey) = dm::crypto::gen_keypair();
        let (spk, sgnkey) = sign::crypto::gen_keypair();
        Self {
            groupkeys: Default::default(),
            msgkey,
            sgnkey,
            pubkey: PublicKey {
                msgs: mpk,
                signs: spk,
            },
        }
    }
}
