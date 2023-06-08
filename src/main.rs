use clap::{Parser, ValueEnum};
use std::io::Write;
use flate2::{Compression, write::GzEncoder};

use std::error::Error;

const EXEC_HEADER: &[u8] = br#"#!/bin/sh
name="$(basename $0)"
which tar 2>&1 >/dev/null || echo "$name: tar is not installed, unable to extract archive" 1>&2
echo "$name: extracting archive..."
tail -n+7 "$0" | tar -xzf -
exit
"#;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use compression
    #[arg(short, long, value_enum)]
    zip: Option<CompressionKind>,

    /// Output archive name
    outfile: String,

    /// Input files
    infiles: Vec<String>,
}

#[derive(Clone, ValueEnum)]
enum CompressionKind {
    /// gzip compression
    Gzip,

    /// zlib compression
    Zlib,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut outfile = std::fs::File::create(args.outfile)?;
    
    let vec = vec![];
    let mut builder = tar::Builder::new(vec);
    for file in args.infiles {
        builder.append_path(file)?;
    }

    let tar = builder.into_inner()?;

    let mut encoder = GzEncoder::new(vec![], Compression::default());
    encoder.write_all(&tar);
    let compressed = encoder.finish()?; 

    let mut archive = vec![];
    archive.extend(EXEC_HEADER.to_vec());
    archive.extend(compressed);

    outfile.write_all(&archive)?;

    Ok(())
}
