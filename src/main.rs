use clap::{Parser, ValueEnum};
use std::io::Write;
use flate2::{Compression, write::GzEncoder, write::ZlibEncoder};

use std::error::Error;
use std::path::Path;
use std::io::Read;

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

    /// Rely on host system tar to extract
    #[arg(short, long)]
    system_tar: bool,

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

    let post_comp;
    if let Some(z) = args.zip {
        post_comp = match z {
            CompressionKind::Gzip => {
                let mut encoder = GzEncoder::new(vec![], Compression::default());
                encoder.write_all(&tar)?;
                encoder.finish()?
            },
            CompressionKind::Zlib => {
                let mut encoder = ZlibEncoder::new(vec![], Compression::default());
                encoder.write_all(&tar)?;
                encoder.finish()?
            }
        };
    } else {
        post_comp = tar;
    }

    let sep = b"##SEP##".to_vec();

    let mut archive = vec![];
    archive.extend(EXEC_HEADER.to_vec());
    if !args.system_tar {
        if Path::new("target/release/unzip").exists() {
            println!("using project unzip");

            let mut unzip = vec![];

            std::fs::File::open("target/release/unzip")?.read_to_end(&mut unzip)?;

            archive.extend(sep.clone());
            archive.extend(unzip);
            archive.extend(sep);
        } else if Path::new("/usr/lib/autoarc/unzip").exists() {
            println!("using system unzip");
        } else {
            eprintln!("can't find unzip; falling back to system tar");
        }
    }
    archive.extend(post_comp);

    outfile.write_all(&archive)?;

    Ok(())
}
