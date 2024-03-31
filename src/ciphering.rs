//! Ciphering methods for en- and decoding a [BoardingPass].
use crate::{
    auth_type::AuthType,
    boarding_pass::{
        payloads::JsonWebToken,
        BoardingPass,
    },
};
use anyhow::anyhow;
use jsonwebtoken::{
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};

/// Methods for encoding and decoding a [BoardingPass].
pub trait Ciphering<BPD, AT, CE>
where
    AT: AuthType,
{
    /// Encodes the given [BoardingPass].
    fn encode(
        &self,
        boarding_pass: &BoardingPass<BPD, AT>,
    ) -> anyhow::Result<CE>;
    /// Decodoes the given [BoardingPass].
    fn decode(
        &self,
        encoded_value: &CE,
    ) -> anyhow::Result<BoardingPass<BPD, AT>>;
}

/// Required to en- and decode a [BoardingPass] that contains a [JsonWebToken] as payload.
#[derive(Clone)]
pub struct JwtCipher {
    enc_key: EncodingKey,
    dec_key: DecodingKey,
}

impl JwtCipher {
    /// Creates a random cipher.
    pub fn random() -> Self {
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
            enc_key: EncodingKey::from_secret(authentication_secret.as_bytes()),
            dec_key: DecodingKey::from_secret(authentication_secret.as_bytes()),
        }
    }
}

impl<AT> Ciphering<JsonWebToken, AT, String> for JwtCipher
where
    AT: AuthType,
{
    fn encode(
        &self,
        boarding_pass: &BoardingPass<JsonWebToken, AT>,
    ) -> Result<String, anyhow::Error> {
        let web_token = jsonwebtoken::encode(
            &Header::default(),
            boarding_pass,
            &self.enc_key,
        )
        .map_err(|e| anyhow!("{e}"))?;
        Ok(web_token)
    }
    fn decode(
        &self,
        encoded_value: &String,
    ) -> Result<BoardingPass<JsonWebToken, AT>, anyhow::Error> {
        let claims = jsonwebtoken::decode::<BoardingPass<JsonWebToken, AT>>(
            encoded_value,
            &self.dec_key,
            &Validation::default(),
        )?;
        Ok(claims.claims)
    }
}
