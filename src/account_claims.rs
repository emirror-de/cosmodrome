use {
    chrono::{Duration, Utc},
    jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation},
    naphtha::{models::account::Account, Model},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountClaims {
    user: Account,
    exp: usize,
}

impl AccountClaims {
    pub fn new(user: &Account, valid: Option<Duration>) -> Self {
        let exp = Utc::now();
        let valid = match valid {
            None => Duration::weeks(1),
            Some(v) => v,
        };
        let exp = exp + valid;
        Self {
            user: user.clone(),
            exp: exp.timestamp() as usize,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.user.get_id()
    }

    pub fn get_email(&self) -> String {
        self.user.email.clone()
    }

    pub fn get_username(&self) -> String {
        self.user.username.clone()
    }

    pub fn get_service(&self) -> String {
        self.user.get_service()
    }

    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let web_token = jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;
        Ok(web_token)
    }

    pub fn decode(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let claims = jsonwebtoken::decode::<AccountClaims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(claims.claims)
    }
}
