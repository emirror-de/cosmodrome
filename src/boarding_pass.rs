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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BoardingPass<Cookie> {
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
        let cookies = request.cookies();

        let auth = cookies.get_private(gate.options.cookie_name());
        let Some(auth) = &auth else {
            return Outcome::Error((
                Status::Unauthorized,
                anyhow!("No auth cookie available"),
            ));
        };
        let user = gate.decode(auth.value());
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
