use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, SaltString},
    Argon2,
};

use crate::domain::services::HashError;

#[tracing::instrument(err, name = "Hashing password", skip(password), level = "debug")]
pub fn hash_password(password: &str) -> Result<String, HashError> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(PasswordHash::generate(Argon2::default(), password, &salt)
        .map_err(|err| HashError::Argon2(err.to_string()))?
        .to_string())
}

#[tracing::instrument(
    err,
    name = "Veryifing password",
    skip(password, password_hash),
    level = "debug"
)]
pub fn verify_password(password: &str, password_hash: &str) -> Result<(), HashError> {
    let hash =
        PasswordHash::new(password_hash).map_err(|err| HashError::Argon2(err.to_string()))?;

    hash.verify_password(&[&Argon2::default()], password)
        .map_err(|e| match e {
            argon2::password_hash::Error::Password => HashError::VerifiyFailed,
            _ => HashError::Argon2(e.to_string()),
        })?;

    Ok(())
}
