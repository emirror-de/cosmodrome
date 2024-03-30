//! A [Gate] is the main entrance to your [rocket]. It provides methods for access control, as well
//! as login and logout.
use std::fmt::Display;

use crate::{
    BoardingPass,
    Passport,
};
use anyhow::anyhow;

pub mod memory;

/// The gate type.
pub trait GateType {}

/// Defines a type of gate that deals with cookies.
#[derive(Debug)]
pub struct Cookie;
impl GateType for Cookie {}

/// Defines a type of gate that deals with bearer tokens.
#[derive(Debug)]
pub struct Bearer;
impl GateType for Bearer {}

/// The security check is responsible for the verification of a user at the [Gate].
pub trait SecurityCheck<C> {
    /// Verifies the given credentials and returns the user passport on success.
    fn verify_credentials(&self, credentials: C) -> anyhow::Result<Passport>;
}

/// Responsible for generating the [BoardingPass] and its corresponding token.
pub trait BoardingPassGenerator<T: GateType> {
    /// Generates the [BoardingPass] based on the given [Passport] (usually returned by [SecurityCheck::verify]).
    fn generate_boarding_pass(
        &self,
        passport: &Passport,
    ) -> anyhow::Result<BoardingPass<T>> {
        BoardingPass::<T>::new(passport)
    }
}

/// If required, the [BoardingPass] can be stored in your storage for later
/// use. This can be a database, cookie or similar.
pub trait BoardingPassStorage<T: GateType, S, I> {
    /// Returns a reference to the [BoardingPass] in your storage.
    fn boarding_pass(
        &self,
        identifier: I,
        storage: S,
    ) -> anyhow::Result<Option<BoardingPass<T>>>;
    /// Stores the given [BoardingPass] in your storage.
    fn store_boarding_pass(
        &self,
        boarding_pass: &BoardingPass<T>,
        storage: S,
    ) -> anyhow::Result<()>;
    /// Removes the [BoardingPass] from your storage.
    fn remove_boarding_pass(
        &self,
        identifier: I,
        storage: S,
    ) -> anyhow::Result<()>;
}

/// Provides an interface an account provider. This can be anything that contains
/// the user information for example a database or a file.
pub trait Gate<C, T: GateType, S, I, EO, EI, ERR>:
    SecurityCheck<C>
    + BoardingPassGenerator<T>
    + BoardingPassStorage<T, S, I>
    + BoardingPassEncoder<T, EO, ERR>
    + BoardingPassDecoder<T, EI, ERR>
where
    ERR: Display,
{
    /// Sets a cookie containing a [`jsonwebtoken`] if the credentials are successfully verified.
    ///
    /// Returns the encoded [BoardingPass] if successful.
    fn login(&self, credentials: C, storage: S) -> anyhow::Result<EO> {
        let passport = self.verify_credentials(credentials)?;
        let boarding_pass = self.generate_boarding_pass(&passport)?;
        //.with_validity(self.options.login_validity().clone());
        self.store_boarding_pass(&boarding_pass, storage)?;
        /*
        let token = boarding_pass
            .encode(settings.authentication_secret())
            .map_err(|e| anyhow!("{e}"))?;
        let cookie =
            RocketCookie::build((settings.cookie_name().to_string(), token))
                .path(settings.cookie_path().to_string())
                .secure(false)
                .http_only(true)
                .same_site(rocket::http::SameSite::None);
        cookies.add_private(cookie);
        Ok(())
        */
        self.encode(&boarding_pass).map_err(|e| anyhow!("{e}"))
    }

    /// Loggs the user out by removing the cookie that contains their
    /// boarding pass.
    fn logout(&self, identifier: I, storage: S) -> anyhow::Result<()> {
        self.remove_boarding_pass(identifier, storage)
    }
}

/// If your [BoardingPass] is too big to carry, the encoder is ready to compress its size. This can be by creating a token or anything else you require.
pub trait BoardingPassEncoder<T: GateType, O, E>
where
    E: Display,
{
    /// Encodes the given [BoardingPass] using the given properties.
    fn encode(&self, boarding_pass: &BoardingPass<T>) -> Result<O, E>;
}

/// If your [BoardingPass] has been encoded, you can decode it with this trait.
pub trait BoardingPassDecoder<T: GateType, I, E>
where
    E: Display,
{
    /// Decodoes the given [BoardingPass] using the given properties.
    fn decode(&self, encoded_value: I) -> Result<BoardingPass<T>, E>;
}
