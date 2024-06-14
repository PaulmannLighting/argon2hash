use argon2::{password_hash::Result, password_hash::SaltString, Argon2, PasswordHasher};
use rand_core::{CryptoRngCore, SeedableRng};

pub struct PwHasher<'key, R>
where
    R: CryptoRngCore,
{
    csprng: R,
    argon2: Argon2<'key>,
}

impl<R> PwHasher<'_, R>
where
    R: CryptoRngCore,
{
    /// Hash a password.
    ///
    /// # Errors
    /// Returns an [`argon2::password_hash::Error`] if hashing fails.
    pub fn hash(&mut self, password: &[u8]) -> Result<String> {
        Ok(self
            .argon2
            .hash_password(password, &SaltString::generate(&mut self.csprng))?
            .to_string())
    }
}

impl<R> Default for PwHasher<'_, R>
where
    R: CryptoRngCore + SeedableRng,
{
    fn default() -> Self {
        Self {
            csprng: R::from_entropy(),
            argon2: Argon2::default(),
        }
    }
}
