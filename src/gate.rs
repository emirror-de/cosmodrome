//! A [Gate] is the main entrance to your [rocket]. It provides methods for access control, as well
//! as login and logout.
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
    passport_register::PassportRegister,
    storage::BoardingPassStorage,
    Ticket,
};
use anyhow::anyhow;

/// A [Gate] is able to verify, grant and deny access to a [rocket].
pub trait Gate<BPD, T, ID, ENC>
where
    T: AuthType,
{
    /// Checks if the given [Ticket] is valid and generates a [BoardingPass] on success.
    fn login<BPS, PR>(
        ticket: Ticket,
        passport_register: &PR,
        boarding_pass_storage: &BPS,
    ) -> anyhow::Result<String>
    where
        BPS: BoardingPassStorage<BPD, T, ID, ENC>,
        PR: PassportRegister;

    /// Executes a logout of the user.
    fn logout<BPS>(
        &self,
        identifier: ID,
        boarding_pass_storage: &BPS,
    ) -> anyhow::Result<()>
    where
        BPS: BoardingPassStorage<BPD, T, ID, ENC>,
    {
        boarding_pass_storage.remove_boarding_pass(identifier)
    }
}

/// A gate where the [BoardingPass] is stored as [jsonwebtoken] in a cookie.
pub struct JwtCookieGate;

impl Gate<JsonWebToken, Cookie, (), String> for JwtCookieGate {
    fn login<BPS, PR>(
        ticket: Ticket,
        passport_register: &PR,
        boarding_pass_storage: &BPS,
    ) -> anyhow::Result<String>
    where
        BPS: BoardingPassStorage<JsonWebToken, Cookie, (), String>,
        PR: PassportRegister,
    {
        let Some(passport) = passport_register.verify_credentials(&ticket)?
        else {
            return Err(anyhow!(
                "No passport found for ticket with id: {}",
                ticket.id
            ));
        };
        let boarding_pass: BoardingPass<JsonWebToken, Cookie> =
            BoardingPass::try_from(&passport)?;
        boarding_pass_storage.store_boarding_pass(&boarding_pass)
    }

    /// In case of [Cookie], there is no identifier required as it is already defined in the storage.
    fn logout<BPS>(
        &self,
        identifier: (),
        boarding_pass_storage: &BPS,
    ) -> anyhow::Result<()>
    where
        BPS: BoardingPassStorage<JsonWebToken, Cookie, (), String>,
    {
        boarding_pass_storage.remove_boarding_pass(identifier)
    }
}

/// A gate where the [BoardingPass] is stored as [jsonwebtoken] in the `Authorization` `Bearer` header.
pub struct JwtBearerGate;

impl Gate<JsonWebToken, Bearer, (), String> for JwtBearerGate {
    fn login<BPS, PR>(
        ticket: Ticket,
        passport_register: &PR,
        boarding_pass_storage: &BPS,
    ) -> anyhow::Result<String>
    where
        BPS: BoardingPassStorage<JsonWebToken, Bearer, (), String>,
        PR: PassportRegister,
    {
        let Some(passport) = passport_register.verify_credentials(&ticket)?
        else {
            return Err(anyhow!(
                "No passport found for ticket with id: {}",
                ticket.id
            ));
        };
        let boarding_pass: BoardingPass<JsonWebToken, Bearer> =
            BoardingPass::try_from(&passport)?;
        boarding_pass_storage.store_boarding_pass(&boarding_pass)
    }

    /// In case of [Cookie], there is no identifier required as it is already defined in the storage.
    fn logout<BPS>(
        &self,
        identifier: (),
        boarding_pass_storage: &BPS,
    ) -> anyhow::Result<()>
    where
        BPS: BoardingPassStorage<JsonWebToken, Bearer, (), String>,
    {
        boarding_pass_storage.remove_boarding_pass(identifier)
    }
}

/*
/// A [Gate] is able to verify, grant and deny access to a [rocket].
pub struct Gate<PR, BPD, AT, CE>
where
    PR: PassportRegister,
    AT: AuthType,
{
    /// The global passport register.
    pub passport_register: PR,
    phantom_bpd: PhantomData<BPD>,
    phantom_ce: PhantomData<CE>,
    phantom_at: PhantomData<AT>,
}

impl<PR, BPD, AT, CE> Gate<PR, BPD, AT, CE>
where
    PR: PassportRegister,
    AT: AuthType,
{
    /// Creates a new [Gate] instance.
    pub fn new(passport_register: PR) -> Self {
        Self {
            passport_register,
            phantom_bpd: PhantomData,
            phantom_ce: PhantomData,
            phantom_at: PhantomData,
        }
    }
}

impl<PR> Gate<PR, JsonWebToken, Cookie, String>
where
    PR: PassportRegister,
{
    /// Checks if the given [Ticket] is valid and generates a [BoardingPass] on success.
    pub fn login<BPS, PR>(
        &self,
        ticket: Ticket,
        boarding_pass_storage: BPS,
    ) -> anyhow::Result<String>
    where
        BPS: BoardingPassStorage<JsonWebToken, Cookie, &'static str, String>,
    {
        let Some(passport) =
            self.passport_register.verify_credentials(&ticket)?
        else {
            return Err(anyhow!(
                "No passport found for ticket with id: {}",
                ticket.id
            ));
        };
        let boarding_pass: BoardingPass<JsonWebToken, Cookie> =
            BoardingPass::try_from(&passport)?;
        boarding_pass_storage.store_boarding_pass(&boarding_pass)
    }
}
*/

/*
/// Provides an interface an account provider. This can be anything that contains
/// the user information for example a database or a file.
pub trait GateTrait<C, T: GateType, S, I, EO, EI, ERR>:
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

*/
