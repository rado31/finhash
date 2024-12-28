use sha2::{Digest, Sha256};
use std::sync::mpsc;
use std::thread;

fn ends_with_zeros(hash: &str, number: u8) -> bool {
    hash.ends_with(&"0".repeat(number.into()))
}

fn create_hash(number: u32) -> String {
    let mut hasher = Sha256::new();
    hasher.update(number.to_string().as_bytes());
    format!("{number}, {:x}", hasher.finalize())
}

pub fn create_hashes(n: u8, f: u8) -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    let num_threads = 4;

    for thread_id in 1..=num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            let mut number = thread_id;

            while tx.send(create_hash(number)).is_ok() {
                number += num_threads;
            }
        });
    }

    let mut counter = 0;
    let mut hashes: Vec<String> = vec![];

    for hash in rx {
        if counter == f {
            break;
        }

        if ends_with_zeros(&hash, n) {
            hashes.push(hash);
            counter += 1;
        }
    }

    hashes
}
