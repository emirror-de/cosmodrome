use crate::{
    account::Account,
    AuthSettings,
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

/// The claims that are stored in the [`jsonwebtoken`].
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AccountClaims {
    pub user: Account,
    exp: usize,
}

impl AccountClaims {
    /// Creates a new claim for the given account with a validity of one week.
    /// Use [with_validity](Self::with_validity) for adjustment of the valid timespan.
    pub fn new(user: &Account) -> anyhow::Result<Self> {
        let exp = Utc::now();
        let valid =
            TimeDelta::try_weeks(1).ok_or(anyhow!("TimeDelta overflow."))?;
        let exp = exp + valid;
        Ok(Self {
            user: user.clone(),
            exp: exp.timestamp() as usize,
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

    pub fn encode(
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

    pub fn decode(
        token: &str,
        secret: &str,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        let claims = jsonwebtoken::decode::<AccountClaims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(claims.claims)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AccountClaims {
    type Error = anyhow::Error;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        let Some(settings) = request.rocket().state::<AuthSettings>() else {
            log::error!(
                "No AuthSettings managed by rocket. Please create an instance \
                 and manage it with rocket."
            );
            return Outcome::Forward(Status::InternalServerError);
        };
        let cookies = request.cookies();

        let auth = cookies.get_private(settings.cookie_name());
        let Some(auth) = &auth else {
            return Outcome::Error((
                Status::Unauthorized,
                anyhow!("No auth cookie available"),
            ));
        };
        let user = AccountClaims::decode(
            auth.value(),
            settings.authentication_secret(),
        );
        match user {
            Err(e) => {
                return Outcome::Error((Status::Unauthorized, anyhow!("{e}")));
            }
            Ok(u) => Outcome::Success(u),
        }
    }
}
