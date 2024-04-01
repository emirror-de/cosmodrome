//! Provides storage implementations for different types of [Gate](crate::gate::Gate)s.
use crate::{
    auth_type::{
        AuthType,
        Bearer,
        Cookie,
    },
    boarding_pass::{
        payloads::JsonWebToken,
        BoardingPass,
    },
    ciphering::{
        Ciphering,
        JwtCipher,
    },
};
use anyhow::anyhow;

use rocket::http::{
    Cookie as RocketCookie,
    CookieJar,
};
use std::marker::PhantomData;

/// If required, the [BoardingPass] can be stored in your storage for later
/// use. This can be a database, cookie or similar.
pub trait BoardingPassStorage<BPD, AT, ID, ENC>
where
    AT: AuthType,
{
    /// Returns a copy of the [BoardingPass] in your storage.
    fn boarding_pass(
        &self,
        identifier: ID,
    ) -> anyhow::Result<Option<BoardingPass<BPD, AT>>>;
    /// Stores the given [BoardingPass] in your storage.
    fn store_boarding_pass(
        &self,
        boarding_pass: &BoardingPass<BPD, AT>,
    ) -> anyhow::Result<ENC>;
    /// Removes the [BoardingPass] from your storage.
    fn remove_boarding_pass(&self, identifier: ID) -> anyhow::Result<()>;
}

/// Options required for the [Storage] to work when used with [Cookie] [AuthType].
pub struct CookieStorageOptions<'a> {
    /// The cookie template that is used to store the [BoardingPass].
    pub cookie_template: RocketCookie<'a>,
}

impl<'a> Default for CookieStorageOptions<'a> {
    fn default() -> Self {
        Self {
            cookie_template: RocketCookie::build((
                "cosmodrome".to_string(),
                "",
            ))
            .path("/")
            .secure(true)
            .same_site(rocket::http::SameSite::Strict)
            .build(),
        }
    }
}

impl<'a> CookieStorageOptions<'a> {
    /// Creates a new instance with the given cookie template.
    pub fn new(cookie_template: RocketCookie<'a>) -> Self {
        Self { cookie_template }
    }
}

/// A default storage for [BoardingPass].
pub struct Storage<S, SO, BPD, AT, C, CE>
where
    C: Ciphering<BPD, AT, CE>,
    AT: AuthType,
{
    storage: S,
    options: SO,
    cipher: C,
    phantom_bpd: PhantomData<BPD>,
    phantom_ce: PhantomData<CE>,
    phantom_at: PhantomData<AT>,
}

impl<S, SO, BPD, AT, C, CE> Storage<S, SO, BPD, AT, C, CE>
where
    C: Ciphering<BPD, AT, CE>,
    AT: AuthType,
{
    /// Creates a new instance.
    pub fn new(storage: S, storage_options: SO, cipher: C) -> Self {
        Self {
            storage,
            options: storage_options,
            cipher,
            phantom_bpd: PhantomData,
            phantom_ce: PhantomData,
            phantom_at: PhantomData,
        }
    }
}

impl BoardingPassStorage<JsonWebToken, Cookie, (), String>
    for Storage<
        &CookieJar<'_>,
        CookieStorageOptions<'static>,
        JsonWebToken,
        Cookie,
        JwtCipher,
        String,
    >
{
    /// In the case of usage with [Cookie](RocketCookie), the identifier is not used. Instead, the
    /// given name of the [cookie_template](CookieStorageOptions::cookie_template) is used.
    fn boarding_pass(
        &self,
        _identifier: (),
    ) -> anyhow::Result<Option<BoardingPass<JsonWebToken, Cookie>>> {
        let Some(boarding_pass) = self
            .storage
            .get_private(self.options.cookie_template.name())
        else {
            return Ok(None);
        };
        let boarding_pass: BoardingPass<JsonWebToken, Cookie> =
            self.cipher.decode(&boarding_pass.value().to_string())?;
        Ok(Some(boarding_pass))
    }
    fn store_boarding_pass(
        &self,
        boarding_pass: &BoardingPass<JsonWebToken, Cookie>,
    ) -> anyhow::Result<String> {
        let token = self
            .cipher
            .encode(boarding_pass)
            .map_err(|e| anyhow!("{e}"))?;
        let mut cookie = self.options.cookie_template.clone();
        cookie.set_value(token.clone());
        self.storage.add_private(cookie);
        Ok(token)
    }
    /// In the case of usage with [Cookie](RocketCookie), the identifier is not used. Instead, the
    /// given name of the [cookie_template](CookieStorageOptions::cookie_template) is used.
    fn remove_boarding_pass(&self, _identifier: ()) -> anyhow::Result<()> {
        let cookie = RocketCookie::build(
            self.options.cookie_template.name().to_string(),
        );
        let cookie = match self.options.cookie_template.path() {
            Some(path) => cookie.path(path.to_string()),
            None => cookie,
        };
        self.storage.remove_private(cookie);
        Ok(())
    }
}

impl<'r> BoardingPassStorage<JsonWebToken, Bearer, (), String>
    for Storage<(), (), JsonWebToken, Bearer, JwtCipher, String>
{
    /// The [BoardingPass] is extracted from the [AUTHORIZATION](http::header::AUTHORIZATION) header.
    fn boarding_pass(
        &self,
        _identifier: (),
    ) -> anyhow::Result<Option<BoardingPass<JsonWebToken, Bearer>>> {
        Ok(None)
    }
    fn store_boarding_pass(
        &self,
        boarding_pass: &BoardingPass<JsonWebToken, Bearer>,
    ) -> anyhow::Result<String> {
        let token = self
            .cipher
            .encode(boarding_pass)
            .map_err(|e| anyhow!("{e}"))?;
        Ok(token)
    }
    /// In the case of usage with [Cookie](RocketCookie), the identifier is not used. Instead, the
    /// given name of the [cookie_template](CookieStorageOptions::cookie_template) is used.
    fn remove_boarding_pass(&self, _identifier: ()) -> anyhow::Result<()> {
        Ok(())
    }
}
