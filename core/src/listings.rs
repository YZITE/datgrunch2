use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

type Listing = BTreeSet<String>;

#[derive(Deserialize, Serialize)]
pub struct Listings {
    pub d: Listing,
    pub g: Listing,
}

fn listdir(base_path: &Path, sub: &str) -> Result<Listing, std::io::Error> {
    let mut ret = Listing::new();
    for entry in WalkDir::new(base_path.join(sub)) {
        let entry = entry?;
        if entry.depth() != 1 {
            continue;
        }
        ret.insert(
            entry
                .path()
                .to_str()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "invalid directory entry found: '{}'",
                            entry.path().display()
                        ),
                    )
                })?
                .to_string(),
        );
    }
    Ok(ret)
}

pub fn update_listings(base_path: &Path) -> Result<(), serde_cbor::Error> {
    let listings = Listings {
        d: listdir(base_path, "d")?,
        g: listdir(base_path, "g")?,
    };

    let p = base_path.join("idx.dat");
    let lf = std::fs::File::create(p)?;
    let mut zstde = zstd::stream::Encoder::new(lf, 10)?;

    serde_cbor::to_writer(&mut zstde, &listings)?;

    use std::io::Write;
    let mut w = zstde.finish()?;
    w.flush()?;
    w.sync_all()?;

    Ok(())
}

pub fn parse_listings<R: std::io::Read>(lst: R) -> Result<Listings, serde_cbor::Error> {
    let zstdd = zstd::stream::Decoder::new(lst)?;
    let cbord: Listings = serde_cbor::from_reader(zstdd)?;
    Ok(cbord)
}
