//! A [BoardingPass] is the piece you need to be granted access to a [rocket].
use super::{
    auth_type::{
        AuthType,
        Bearer,
        Cookie,
    },
    ciphering::{
        Ciphering,
        JwtCipher,
    },
    passport::Passport,
    storage::{
        BoardingPassStorage,
        CookieStorageOptions,
        Storage,
    },
};
use anyhow::anyhow;
use chrono::TimeDelta;
use log::error;
use payloads::JsonWebToken;
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

pub mod payloads;

/// The [BoardingPass] is your access card to a [rocket].
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BoardingPass<BPD, T: AuthType> {
    /// The actual boarding pass data, simultaneously defining the type of [BoardingPass].
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

impl TryFrom<&Passport> for BoardingPass<JsonWebToken, Bearer> {
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BoardingPass<JsonWebToken, Bearer> {
    type Error = anyhow::Error;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        let Some(cipher) = request.rocket().state::<JwtCipher>() else {
            log::error!(
                "No JwtCipher managed by rocket. Please create an instance \
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
        let Some(user) = Bearer::extract_value(auth, None) else {
            return Outcome::Error((
                Status::Unauthorized,
                anyhow!("Not a valid Bearer authorization header."),
            ));
        };
        let user = cipher.decode(&user);
        match user {
            Err(e) => {
                return Outcome::Error((Status::Unauthorized, anyhow!("{e}")));
            }
            Ok(u) => Outcome::Success(u),
        }
    }
}
