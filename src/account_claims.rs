use crate::account::Account;
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
use rocket::serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AccountClaims {
    user: Account,
    exp: usize,
}

impl AccountClaims {
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

    pub fn get_id(&self) -> String {
        self.user.id.clone()
    }

    pub fn get_service(&self) -> String {
        self.user.get_service()
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
