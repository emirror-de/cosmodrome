pub use boarding_pass::BoardingPass;
use chrono::TimeDelta;
pub use immigration::{
    Immigration,
    MemoryImmigration,
};
pub use passport::{
    Passport,
    PassportType,
};
pub use ticket::Ticket;

mod boarding_pass;
mod immigration;
mod passport;
mod ticket;

/// Global settings required to run the web auth implementation.
pub struct AirportSettings {
    /// The cookie name where the [AccountClaims](account_claims::AccountClaims) should be stored.
    cookie_name: String,
    /// Defines the path where the cookie is valid.
    cookie_path: String,
    /// The authentication secret used to
    authentication_secret: String,
    /// Defines how long the login is valid.
    login_validity: TimeDelta,
}

impl AirportSettings {
    /// Creates a new settings instance.
    pub fn new(cookie_name: &str, authentication_secret: &str) -> Self {
        Self {
            cookie_name: cookie_name.to_string(),
            authentication_secret: authentication_secret.to_string(),
            login_validity: TimeDelta::try_weeks(1).unwrap(),
            cookie_path: "/".to_string(),
        }
    }

    /// Creates a new settings instance with a randomly generated secret.
    pub fn new_with_random_secret(cookie_name: &str) -> Self {
        use rand::{
            distributions::Alphanumeric,
            thread_rng,
            Rng,
        };

        let authentication_secret: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(60)
            .map(char::from)
            .collect();

        Self {
            cookie_name: cookie_name.to_string(),
            authentication_secret,
            login_validity: TimeDelta::try_weeks(1).unwrap(),
            cookie_path: "/".to_string(),
        }
    }

    /// Returns the authentication secret that is used to de-/encode the [jsonwebtoken].
    pub fn authentication_secret(&self) -> &str {
        &self.authentication_secret
    }

    /// Returns the cookie name that is used to store the [jsonwebtoken].
    pub fn cookie_name(&self) -> &str {
        &self.cookie_name
    }

    /// Returns the cookie path.
    pub fn cookie_path(&self) -> &str {
        &self.cookie_path
    }

    /// How long a login should be valid.
    pub fn login_validity(&self) -> &TimeDelta {
        &self.login_validity
    }
}
