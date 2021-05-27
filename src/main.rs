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

    for fname in fnames {
        if !metadata(&fname).map(|md| md.is_file()).unwrap_or_default() {
            // skip non-files
            continue;
        }
        match read(&fname) {
            Ok(bytes) => {
                let bf = ByteFreq::from_bytes(&bytes);
                println!(
                    "[OK]\t{}\t=>\tbytes: {},\tentropy: {:.4}",
                    fname,
                    bf.total_bytes(),
                    bf.entropy()
                );
            }
            Err(e) => println!("[ERR]\t{}\t=>\t{}", fname, e),
        }
    }
}
