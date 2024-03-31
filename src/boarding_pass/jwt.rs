use crate::passport::Passport;
use chrono::{
    TimeDelta,
    Utc,
};
use rocket::serde::{
    Deserialize,
    Serialize,
};

/// A [jsonwebtoken] [BoardingPass] implementation.
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct JsonWebToken {
    /// The user passport.
    pub passport: Passport,
    exp: usize,
}

impl JsonWebToken {
    /// Creates a new claim for a [jsonwebtoken].
    pub fn new(passport: &Passport, valid_timespan: TimeDelta) -> Self {
        let exp = Utc::now() + valid_timespan;
        Self {
            passport: passport.to_owned(),
            exp: exp.timestamp() as usize,
        }
    }

    /// Returns `true` if the token is still valid.
    pub fn is_valid(&self) -> bool {
        self.exp > Utc::now().timestamp() as usize
    }
}
