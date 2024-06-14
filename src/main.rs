use clap::Parser;
use rand_chacha::ChaCha20Rng;

use argon2hash::{Args, PwHasher, HASH_ERROR};

fn main() {
    env_logger::init();
    let args = Args::parse();
    let mut hasher = PwHasher::<ChaCha20Rng>::default();

    if args.is_base64() {
        args.b64content().for_each(|(line, bytes)| {
            println!(
                "{line}\t{}",
                hasher.hash(bytes.as_slice()).expect(HASH_ERROR)
            );
        });
    } else {
        args.content().lines().for_each(|line| {
            println!(
                "{line}\t{}",
                hasher.hash(line.as_bytes()).expect(HASH_ERROR)
            );
        });
    }
}
