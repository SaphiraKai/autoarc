use clap::Parser;

use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    outfile: String,
    infiles: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>>{
    let args = Args::parse();

    let mut archive = tar::Builder::new(vec![]);
    for file in args.infiles {
        archive.append_path(file);
    }

    Ok(())
}
