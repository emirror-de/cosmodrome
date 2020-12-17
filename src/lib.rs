use {
    dotenv::dotenv,
    std::env,
};

pub use {
    user_claims::UserClaims,
    jsonwebtoken,
};

mod user_claims;
mod rocket;

const ENV_COOKIE_NAME: &str = "WEBAUTH_COOKIE_NAME";
const ENV_AUTHENTICATION_SECRET: &str = "WEBAUTH_AUTHENTICATION_SECRET";

pub fn get_authentication_secret() -> Result<String, std::env::VarError> {
    dotenv().ok();
    env::var(crate::ENV_AUTHENTICATION_SECRET)
}

pub fn get_cookie_name() -> Result<String, std::env::VarError> {
    dotenv().ok();
    env::var(crate::ENV_COOKIE_NAME)
}
