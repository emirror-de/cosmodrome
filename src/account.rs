//! Account data model.
mod account_type;

pub use account_type::AccountType;
use anyhow::anyhow;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        Encoding,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2,
};
use chrono::prelude::*;
use rocket::serde::{
    Deserialize,
    Serialize,
};

/// Defines a user account of a service.
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    /// The unique id of the account. This can be for example the username or email.
    pub id: String,
    /// Password to login to the service. This is never stored plain text.
    password: String,
    /// Service name the account is valid for.
    service: String,
    /// Type of this account.
    pub account_type: AccountType,
    /// Wether the account is disabled.
    pub disabled: bool,
    /// Whether the account has been confirmed. This is useful in combination with for example
    /// E-Mail verficiation.
    pub confirmed: bool,
    /// Determines when this account expires.
    pub expires_at: DateTime<Utc>,
}

impl Account {
    /// Creates a new user account with [Account::disabled] and [Account::confirmed] set to `false`.
    pub fn new(
        id: String,
        password: String,
        service: String,
        account_type: AccountType,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            id,
            password: Self::hash_password(password)?,
            service: service,
            account_type,
            disabled: false,  // always activate
            confirmed: false, // always require user to confirm it
            expires_at: chrono::Utc::now(),
        })
    }

    /// Returns the service this account belongs to.
    pub fn get_service(&self) -> String {
        self.service.clone()
    }

    /// Saves the ```new_password``` to the struct after verifying the ```old_password```.
    /// Does NOT automatically call the ```update``` function to update the database.
    pub fn change_password(
        &mut self,
        old_password: String,
        new_password: String,
    ) -> anyhow::Result<()> {
        if self.verify_password(old_password)? {
            self.password = Self::hash_password(new_password)?;
            Ok(())
        } else {
            Err(anyhow!("Passwords do not match."))
        }
    }

    /// Checks if the given password is correct.
    pub fn verify_password(&self, password: String) -> anyhow::Result<bool> {
        let hash = PasswordHash::parse(&self.password, Encoding::B64)
            .map_err(|e| anyhow!("{e}"))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok())
    }

    /// Hashes the password using `[argon2]`.
    fn hash_password(password: String) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("{e}"))?
            .to_string())
    }
}
