use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use rand::rngs::OsRng;

pub fn hash_password(password: &String) -> String {
    // Generate a random salt
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    // Create an Argon2 instance
    let argon2 = Argon2::default();
    // Hash the password
    let hash = argon2
        .hash_password(password, &salt)
        .expect("Failed to hash password");
    // Print the hashed password
    return hash.to_string();
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed);
    if parsed_hash.is_err() {
        return false;
    }

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash.unwrap())
        .is_ok()
}