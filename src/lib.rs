use base64::alphabet::STANDARD;
use base64::engine::general_purpose::NO_PAD;
use base64::engine::GeneralPurpose;

pub use args::Args;
pub use pw_hasher::PwHasher;

mod args;
mod pw_hasher;

pub const BASE64: GeneralPurpose = GeneralPurpose::new(&STANDARD, NO_PAD);
pub const HASH_ERROR: &str = "Could not hash data";
