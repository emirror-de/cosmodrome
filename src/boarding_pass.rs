use crate::{
    gate::{
        memory::MemoryGate,
        Bearer,
        Cookie,
        GateType,
    },
    passport::Passport,
};
use anyhow::anyhow;
use chrono::{
    TimeDelta,
    Utc,
};
use jsonwebtoken::{
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};
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

/// The claims that are stored in the [`jsonwebtoken`].
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BoardingPass<T: GateType> {
    /// The passport of the user.
    pub user: Passport,
    exp: usize,
    #[serde(skip)]
    phantom_data: PhantomData<T>,
}

impl<T: GateType> BoardingPass<T> {
    /// Creates a new claim for the given account with a validity of one week.
    /// Use [with_validity](Self::with_validity) for adjustment of the valid timespan.
    pub fn new(user: &Passport) -> anyhow::Result<Self> {
        let exp = Utc::now();
        let valid =
            TimeDelta::try_weeks(1).ok_or(anyhow!("TimeDelta overflow."))?;
        let exp = exp + valid;
        Ok(Self {
            user: user.clone(),
            exp: exp.timestamp() as usize,
            phantom_data: PhantomData,
        })
    }

    /// Sets the validity of the login.
    pub fn with_validity(self, value: TimeDelta) -> Self {
        let exp = Utc::now() + value;
        Self {
            exp: exp.timestamp() as usize,
            ..self
        }
    }

    /// `False` if the login is still valid.
    pub fn is_login_expired(&self) -> bool {
        self.exp > Utc::now().timestamp() as usize
    }
}

impl<T: GateType>
    BoardingPassEncoder<T, String, &str, jsonwebtoken::errors::Error>
    for BoardingPass<T>
{
    /// Encodes the boarding pass with the given secret into a
    /// [jsonwebtoken].
    fn encode(
        &self,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let web_token = jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;
        Ok(web_token)
    }
}

impl<T: GateType>
    BoardingPassDecoder<T, &str, &str, jsonwebtoken::errors::Error>
    for BoardingPass<T>
{
    /// Decodes the boarding pass with the given secret from a
    /// [jsonwebtoken].
    fn decode(
        token: &str,
        secret: &str,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        let claims = jsonwebtoken::decode::<Self>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(claims.claims)
    }
}

/// If your [BoardingPass] is too big to carry, the encoder is ready to compress its size. This can be by creating a token or anything else you require.
pub trait BoardingPassEncoder<T: GateType, O, P, E> {
    /// Encodes the given [BoardingPass] using the given properties.
    fn encode(&self, properties: P) -> Result<O, E>;
}

/// If your [BoardingPass] has been encoded, you can decode it with this trait.
pub trait BoardingPassDecoder<T: GateType, I, P, E> {
    /// Decodoes the given [BoardingPass] using the given properties.
    fn decode(encoded_value: I, properties: P) -> Result<BoardingPass<T>, E>;
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BoardingPass<Cookie> {
    type Error = anyhow::Error;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        let Some(gate) = request.rocket().state::<MemoryGate>() else {
            log::error!(
                "No MemoryGate managed by rocket. Please create an instance \
                 and manage it with rocket."
            );
            return Outcome::Forward(Status::InternalServerError);
        };
        let cookies = request.cookies();

        let auth = cookies.get_private(gate.options.cookie_name());
        let Some(auth) = &auth else {
            return Outcome::Error((
                Status::Unauthorized,
                anyhow!("No auth cookie available"),
            ));
        };
        let user = BoardingPass::decode(
            auth.value(),
            gate.options.authentication_secret(),
        );
        match user {
            Err(e) => {
                return Outcome::Error((Status::Unauthorized, anyhow!("{e}")));
            }
            Ok(u) => Outcome::Success(u),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BoardingPass<Bearer> {
    type Error = anyhow::Error;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, Self::Error> {
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
            BoardingPass::decode(token, gate.options.authentication_secret())
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
