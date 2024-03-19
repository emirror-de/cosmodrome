use crate::{
    Account,
    AccountClaims,
    AccountCredentials,
    AuthSettings,
};
use anyhow::anyhow;
use rocket::http::{
    Cookie,
    CookieJar,
};

/// Provides an interface an account provider. This can be anything that contains
/// the user information for example a database or a file.
pub trait AccountProvider<C> {
    /// Verifies the given credentials and returns the user account on success.
    fn verify(&self, credentials: C) -> anyhow::Result<Account>;

    /// Sets a cookie containing a [`jsonwebtoken`] if the credentials are successfully verified.
    fn login(
        &self,
        credentials: C,
        settings: &AuthSettings,
        cookies: &CookieJar<'_>,
    ) -> anyhow::Result<()> {
        let account = self.verify(credentials)?;
        let claims = AccountClaims::new(&account)?
            .with_validity(settings.login_validity().clone());
        let token = claims
            .encode(settings.authentication_secret())
            .map_err(|e| anyhow!("{e}"))?;
        let cookie = Cookie::build((settings.cookie_name().to_string(), token))
            .path(settings.cookie_path().to_string());
        cookies.add_private(cookie);
        Ok(())
    }

    /// Removes the cookie if available.
    fn logout(&self, settings: &AuthSettings, cookies: &CookieJar<'_>) {
        cookies.remove_private(
            Cookie::build(settings.cookie_name().to_string())
                .path(settings.cookie_path().to_string()),
        );
    }
}

/// Provides a list of accounts from memory.
pub struct MemoryAccountProvider {
    account_list: Vec<Account>,
}

impl AccountProvider<AccountCredentials> for MemoryAccountProvider {
    fn verify(
        &self,
        credentials: AccountCredentials,
    ) -> anyhow::Result<Account> {
        let account = self
            .account_list
            .iter()
            .find(|a| a.id == credentials.id)
            .map(|a| a.to_owned())
            .ok_or(anyhow!("User not found."))?;
        if account.verify_password(&credentials.secret)? {
            Ok(account)
        } else {
            Err(anyhow!("Invalid credentials."))
        }
    }
}

impl From<Vec<Account>> for MemoryAccountProvider {
    fn from(account_list: Vec<Account>) -> Self {
        Self { account_list }
    }
}
