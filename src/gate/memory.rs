//! Implementations of a [Gate] that lives in memory.
use super::{
    Bearer,
    BoardingPassDecoder,
    BoardingPassEncoder,
    BoardingPassGenerator,
    BoardingPassStorage,
    Cookie,
    Gate,
    GateType,
    SecurityCheck,
};
use crate::{
    BoardingPass,
    Passport,
    Ticket,
};
use anyhow::anyhow;
use chrono::TimeDelta;
use jsonwebtoken::{
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};
use rocket::http::{
    Cookie as RocketCookie,
    CookieJar,
};

/// Options required for the [MemoryGate] to work.
pub struct MemoryGateOptions {
    /// The cookie name where the [BoardingPass](boarding_pass::BoardingPass) will be stored. Defaults to `cosmodrome`.
    cookie_name: String,
    /// Defines the path where the cookie is valid.
    ///
    /// **Default:**  `/`
    cookie_path: String,
    /// The authentication secret used to encrypt the [jsonwebtoken].
    ///
    /// **Default:**  60 character random key.
    authentication_secret: String,
    /// Defines how long the [BoardingPass](boarding_pass::BoardingPass) is valid.
    ///
    /// **Default:**  One week.
    login_validity: TimeDelta,
}

impl Default for MemoryGateOptions {
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
            cookie_name: "cosmodrome".to_string(),
            cookie_path: "/".to_string(),
            authentication_secret,
            login_validity: TimeDelta::try_weeks(1)
                .expect("Could not create TimeDelta with value of one week!"),
        }
    }
}

impl MemoryGateOptions {
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

/// Provides a list of accounts from memory.
pub struct MemoryGate {
    account_list: Vec<Passport>,
    /// The options for this gate.
    pub options: MemoryGateOptions,
}

impl SecurityCheck<Ticket> for MemoryGate {
    fn verify_credentials(
        &self,
        credentials: Ticket,
    ) -> anyhow::Result<Passport> {
        let account = self
            .account_list
            .iter()
            .find(|a| a.id == credentials.id)
            .map(|a| a.to_owned())
            .ok_or(anyhow!("User not found."))?;
        if account.verify_password(&credentials.secret)? {
            Ok(account)
        } else {
            Err(anyhow!("Invalid credentials."))
        }
    }
}
impl BoardingPassGenerator<Cookie> for MemoryGate {}
impl BoardingPassStorage<Cookie, &CookieJar<'_>, ()> for MemoryGate {
    fn boarding_pass(
        &self,
        _identifier: (),
        storage: &CookieJar<'_>,
    ) -> anyhow::Result<Option<BoardingPass<Cookie>>> {
        let Some(boarding_pass) =
            storage.get_private(self.options.cookie_name())
        else {
            return Ok(None);
        };
        let boarding_pass: BoardingPass<Cookie> =
            self.decode(boarding_pass.value())?;
        Ok(Some(boarding_pass))
    }
    fn store_boarding_pass(
        &self,
        boarding_pass: &BoardingPass<Cookie>,
        storage: &CookieJar<'_>,
    ) -> anyhow::Result<()> {
        let token = self.encode(boarding_pass).map_err(|e| anyhow!("{e}"))?;
        let cookie = RocketCookie::build((
            self.options.cookie_name().to_string(),
            token,
        ))
        .path(self.options.cookie_path().to_string())
        .secure(false)
        .http_only(true)
        .same_site(rocket::http::SameSite::None);
        storage.add_private(cookie);
        Ok(())
    }
    fn remove_boarding_pass(
        &self,
        _identifier: (),
        storage: &CookieJar,
    ) -> anyhow::Result<()> {
        storage.remove_private(
            RocketCookie::build(self.options.cookie_name().to_string())
                .path(self.options.cookie_path().to_string()),
        );
        Ok(())
    }
}
impl
    Gate<
        Ticket,
        Cookie,
        &CookieJar<'_>,
        (),
        String,
        &str,
        jsonwebtoken::errors::Error,
    > for MemoryGate
{
}

impl From<(Vec<Passport>, MemoryGateOptions)> for MemoryGate {
    fn from(value: (Vec<Passport>, MemoryGateOptions)) -> Self {
        Self {
            account_list: value.0,
            options: value.1,
        }
    }
}

impl BoardingPassGenerator<Bearer> for MemoryGate {}
impl BoardingPassStorage<Bearer, (), ()> for MemoryGate {
    fn boarding_pass(
        &self,
        _identifier: (),
        _storage: (),
    ) -> anyhow::Result<Option<BoardingPass<Bearer>>> {
        Ok(None)
    }
    fn store_boarding_pass(
        &self,
        _boarding_pass: &BoardingPass<Bearer>,
        _storage: (),
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn remove_boarding_pass(
        &self,
        _identifier: (),
        _storage: (),
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
impl Gate<Ticket, Bearer, (), (), String, &str, jsonwebtoken::errors::Error>
    for MemoryGate
{
}

impl<T: GateType> BoardingPassEncoder<T, String, jsonwebtoken::errors::Error>
    for MemoryGate
{
    /// Encodes the boarding pass with the given secret into a
    /// [jsonwebtoken].
    fn encode(
        &self,
        boarding_pass: &BoardingPass<T>,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let web_token = jsonwebtoken::encode(
            &Header::default(),
            boarding_pass,
            &EncodingKey::from_secret(
                self.options.authentication_secret().as_bytes(),
            ),
        )?;
        Ok(web_token)
    }
}

impl<T: GateType> BoardingPassDecoder<T, &str, jsonwebtoken::errors::Error>
    for MemoryGate
{
    /// Decodes the boarding pass with the given secret from a
    /// [jsonwebtoken].
    fn decode(
        &self,
        token: &str,
    ) -> Result<BoardingPass<T>, jsonwebtoken::errors::Error> {
        let claims = jsonwebtoken::decode::<BoardingPass<T>>(
            &token,
            &DecodingKey::from_secret(
                self.options.authentication_secret().as_bytes(),
            ),
            &Validation::default(),
        )?;
        Ok(claims.claims)
    }
}
