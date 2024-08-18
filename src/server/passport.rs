//! Passports are the identification card for a user. Traditionally known as `Account`.
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
use chrono::{
    DateTime,
    TimeDelta,
    Utc,
};
pub use passport_type::PassportType;
use rocket::serde::{
    Deserialize,
    Serialize,
};

mod passport_type;

/// Defines a passport of a user.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Passport {
    /// The unique id of the passport. For example username, email or some string of your choice.
    pub id: String,
    /// Password to login to the service. Resides encoded in memory.
    password: String,
    /// A list of scopes that the user can access.
    services: Vec<String>,
    /// Type of this passport.
    pub account_type: PassportType,
    /// Wether the passport is disabled.
    pub disabled: bool,
    /// Whether the passport has been confirmed. This is useful in combination
    /// with for example E-Mail verficiation.
    pub confirmed: bool,
    /// Determines when this passport expires.
    pub expires_at: DateTime<Utc>,
}

impl Passport {
    /// Creates a new passport with [Passport::disabled] and [Passport::confirmed] set to `false`.
    pub fn new(
        id: &str,
        password: &str,
        services: &[&str],
        account_type: PassportType,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            id: id.to_string(),
            password: Self::hash_password(password)?,
            services: services
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            account_type,
            disabled: false,  // always activate
            confirmed: false, // always require user to confirm it
            expires_at: chrono::Utc::now()
                + TimeDelta::try_weeks(104).ok_or(anyhow!(
                    "Internal server error. Could not create TimeDelta with \
                     two years."
                ))?,
        })
    }

    /// Returns the services this passport is valid for.
    pub fn services(&self) -> &[String] {
        &self.services
    }

    /// Saves the ```new_password``` to the struct after verifying the ```old_password```.
    /// Does NOT automatically call the ```update``` function to update the database.
    pub fn change_password(
        &mut self,
        old_password: &str,
        new_password: &str,
    ) -> anyhow::Result<()> {
        if self.verify_password(old_password)? {
            self.password = Self::hash_password(new_password)?;
            Ok(())
        } else {
            Err(anyhow!("Passwords do not match."))
        }
    }

    /// Checks if the given password is correct.
    pub fn verify_password(&self, password: &str) -> anyhow::Result<bool> {
        let hash = PasswordHash::parse(&self.password, Encoding::B64)
            .map_err(|e| anyhow!("{e}"))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok())
    }

    /// Hashes the password using `[argon2]`.
    fn hash_password(password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!("{e}"))?
            .to_string())
    }
}
