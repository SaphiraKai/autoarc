use tar::Archive;
use flate2::{read::GzDecoder};

fn main() -> std::io::Result<()> {
    let decoder = GzDecoder::new(std::io::stdin());
    let mut archive = Archive::new(decoder);
    archive.unpack("./autoarc-out/")?;

    Ok(())
}