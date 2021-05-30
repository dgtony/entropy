mod freq;

use freq::ByteFreq;
use std::env;
use std::fs::{metadata, read};
use std::process::exit;

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
        match read(&fname) {
            Ok(bytes) => {
                let bf = ByteFreq::from_bytes(&bytes);
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
