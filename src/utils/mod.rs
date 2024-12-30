use crate::cli::Params;
use sha2::{Digest, Sha256};
use std::sync::mpsc;
use std::thread;

fn ends_with_zeros(hash: &str, number: u32) -> bool {
    hash.ends_with("0".repeat(number as usize).as_str())
}

fn create_hash(number: u32) -> String {
    let mut hasher = Sha256::new();
    hasher.update(number.to_string().as_bytes());
    format!("{number}, {:x}", hasher.finalize())
}

pub fn create_hashes(args: Params) -> Vec<String> {
    let (tx, rx) = mpsc::channel();

    for thread_id in 1..=args.threads {
        let tx = tx.clone();

        thread::spawn(move || {
            let mut number = thread_id;

            while tx.send(create_hash(number)).is_ok() {
                number += args.threads;
            }
        });
    }

    let mut counter = 0;
    let mut hashes: Vec<String> = vec![];

    for hash in rx {
        if counter == args.f {
            break;
        }

        if ends_with_zeros(&hash, args.n) {
            hashes.push(hash);
            counter += 1;
        }
    }

    hashes
}
