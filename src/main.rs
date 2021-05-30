mod freq;
mod reader;

use freq::ByteFreq;
use reader::ChunkReader;
use std::env;
use std::fs::{metadata, OpenOptions};
use std::process::exit;

// File buffer: 64Mb
const BUFFER_SIZE: usize = 1 << 26;

fn main() {
    // files to be processed
    let fnames: Vec<String> = env::args().skip(1).collect();
    if fnames.is_empty() {
        eprintln!("Computing entropy for the files: provide some, plz.");
        exit(1);
    }

    // align filenames by the longest path
    let fname_width = fnames
        .iter()
        .map(|n| n.len())
        .max()
        .expect("fnames not empty");

    // compute entropy for the files one-by-one
    for fname in fnames {
        if !metadata(&fname).map(|md| md.is_file()).unwrap_or_default() {
            // skip non-files
            continue;
        }

        match OpenOptions::new().read(true).open(&fname) {
            Ok(file) => {
                let reader = ChunkReader::new(file, BUFFER_SIZE);
                let bf: ByteFreq = reader.collect();
                println!(
                    "[OK]  {:<width$} => entropy: {:.5}, bytes: {}",
                    fname,
                    bf.entropy(),
                    bf.total_bytes(),
                    width = fname_width,
                );
            }
            Err(e) => eprintln!("[ERR] {:<width$} => {}", fname, e, width = fname_width),
        }
    }
}
