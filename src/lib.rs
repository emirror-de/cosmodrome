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

/// Required configuration values to run an airport.
pub struct AirportConfig {
    /// The cookie name where the [BoardingPass](boarding_pass::BoardingPass) will be stored. Defaults to `rocket-airport`.
    cookie_name: String,
    /// Defines the path where the cookie is valid. Defaults to `/`
    cookie_path: String,
    /// The authentication secret used to encrypt the [jsonwebtoken].
    /// Default value is a 60 character random key.
    authentication_secret: String,
    /// Defines how long the [BoardingPass](boarding_pass::BoardingPass) is valid. Defaults to one week.
    login_validity: TimeDelta,
}

impl Default for AirportConfig {
    fn default() -> Self {
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
            cookie_name: "rocket-airport".to_string(),
            cookie_path: "/".to_string(),
            authentication_secret,
            login_validity: TimeDelta::try_weeks(1)
                .expect("Could not create TimeDelta with value of one week!"),
        }
    }
}

impl AirportConfig {
    /// Creates a new airport configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the cookie name.
    pub fn with_cookie_name(self, cookie_name: &str) -> Self {
        Self {
            cookie_name: cookie_name.to_string(),
            ..self
        }
    }

    /// Sets the cookie path where the [BoardingPass](boarding_pass::BoardingPass) is valid.
    pub fn with_cookie_path(self, cookie_path: &str) -> Self {
        Self {
            cookie_path: cookie_path.to_string(),
            ..self
        }
    }

    /// Sets the secret key that is used to encrypt the [jsonwebtoken].
    pub fn with_authentication_secret(
        self,
        authentication_secret: &str,
    ) -> Self {
        Self {
            authentication_secret: authentication_secret.to_string(),
            ..self
        }
    }

    /// Sets how long a [BoardingPass](boarding_pass::BoardingPass) is be valid.
    pub fn with_login_validity(self, login_validity: TimeDelta) -> Self {
        Self {
            login_validity,
            ..self
        }
    }

    /// Returns the authentication secret that is used to de-/encode the [jsonwebtoken].
    pub(crate) fn authentication_secret(&self) -> &str {
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

    /// How long a [BoardingPass](boarding_pass::BoardingPass) is be valid.
    pub fn login_validity(&self) -> &TimeDelta {
        &self.login_validity
    }
}
