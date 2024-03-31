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
}
