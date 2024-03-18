pub use account_claims::AccountClaims;
pub use jsonwebtoken;

pub mod account;
mod account_claims;
mod rocket;

/// Global settings required to run the web auth implementation.
pub struct AuthSettings {
    /// The cookie name where the [AccountClaims](account_claims::AccountClaims) should be stored.
    cookie_name: String,
    /// The authentication secret used to
    authentication_secret: String,
}

impl AuthSettings {
    /// Creates a new settings instance.
    pub fn new(cookie_name: &str, authentication_secret: &str) -> Self {
        Self {
            cookie_name: cookie_name.to_string(),
            authentication_secret: authentication_secret.to_string(),
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
}
