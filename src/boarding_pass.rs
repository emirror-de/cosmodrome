//! A [BoardingPass] is the piece you need to be granted access to a [rocket].
use crate::{
    auth_type::{
        AuthType,
        Cookie,
    },
    gate::Gate,
    passport::Passport,
};
use anyhow::anyhow;
use chrono::{
    TimeDelta,
    Utc,
};
pub use ciphering::{
    Ciphering,
    JwtCipher,
};
pub use jwt::JsonWebToken;
use log::error;
use rocket::{
    http::Status,
    request::{
        FromRequest,
        Outcome,
        Request,
    },
    serde::{
        Deserialize,
        Serialize,
    },
};
use std::marker::PhantomData;
pub use storage::{
    BoardingPassStorage,
    CookieStorageOptions,
    Storage,
};

pub mod ciphering;
mod jwt;
mod storage;

/// The [BoardingPass] is your access card to a [rocket].
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BoardingPass<BPD, T: AuthType> {
    /// The actual boarding pass data, simultaneously defining the type of [BoardinPass].
    #[serde(flatten)]
    pub data: BPD,
    #[serde(skip)]
    phantom_auth: PhantomData<T>,
}

impl TryFrom<&Passport> for BoardingPass<JsonWebToken, Cookie> {
    type Error = anyhow::Error;
    fn try_from(value: &Passport) -> Result<Self, Self::Error> {
        let valid =
            TimeDelta::try_weeks(1).ok_or(anyhow!("TimeDelta overflow."))?;
        Ok(Self {
            data: JsonWebToken::new(value, valid),
            phantom_auth: PhantomData,
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BoardingPass<JsonWebToken, Cookie> {
    type Error = anyhow::Error;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        let Some(cipher) = request.rocket().state::<JwtCipher>() else {
            log::error!(
                "No cosmodrome JwtCipher managed by rocket. Please create an \
                 instance and manage it with rocket."
            );
            return Outcome::Forward(Status::InternalServerError);
        };
        let storage = Storage::new(
            request.cookies(),
            CookieStorageOptions::default(),
            cipher.clone(),
        );
        let user = match storage.boarding_pass(()) {
            Err(e) => {
                error!("{e}");
                return Outcome::Forward(Status::InternalServerError);
            }
            Ok(u) => u,
        };
        match user {
            None => Outcome::Error((
                Status::Unauthorized,
                anyhow!("User not found."),
            )),
            Some(u) => Outcome::Success(u),
        }
    }
}

/*
#[rocket::async_trait]
impl<'r> FromRequest<'r> for BoardingPassOld<Bearer> {
    type Error = anyhow::Error;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        use crate::gate::BoardingPassDecoder;
        let Some(gate) = request.rocket().state::<MemoryGate>() else {
            log::error!(
                "No MemoryGate managed by rocket. Please create an instance \
                 and manage it with rocket."
            );
            return Outcome::Forward(Status::InternalServerError);
        };
        let headers = request.headers();

        let Some(auth) = headers.get_one("Authorization") else {
            return Outcome::Error((
                Status::Unauthorized,
                anyhow!("No Authorization header available."),
            ));
        };
        let user = if auth.starts_with("Bearer ") {
            let token = auth.strip_prefix("Bearer ").unwrap();
            gate.decode(token)
        } else {
            return Outcome::Error((
                Status::Unauthorized,
                anyhow!("Not a valid Bearer authorization header."),
            ));
        };
        match user {
            Err(e) => {
                return Outcome::Error((Status::Unauthorized, anyhow!("{e}")));
            }
            Ok(u) => Outcome::Success(u),
        }
    }
}
*/
