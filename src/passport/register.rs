use std::collections::HashMap;

use crate::{
    passport::Passport,
    Ticket,
};
use anyhow::anyhow;
use log::debug;

/// The passport register contains a collection of passports that are
/// known to your application.
///
/// `ID` is the unique identifier type for a [Passport].
pub trait PassportRegister {
    /// Returns the passport for the given `passport_id`.
    fn passport(&self, passport_id: &str) -> anyhow::Result<Option<Passport>>;
    /// Stores the given passport in the register returning its ID for further usage.
    fn set_passport(&mut self, passport: Passport) -> anyhow::Result<String>;
    /// Verifies if the given [Ticket] is valid.
    /// Return scenarios should be the following:
    /// - If valid, a copy of the corresponding passport is returned.
    /// - If no corresponding [Passport] is found, the return value should be `Ok(None)`.
    /// - In all other cases, it should return `Err(_)`.
    fn verify_credentials(
        &self,
        ticket: &Ticket,
    ) -> anyhow::Result<Option<Passport>>;
}

/// A [MemoryPassportRegister] is a data structure where all [Passport]s are stored in memory.
pub struct MemoryPassportRegister {
    passports: HashMap<String, Passport>,
}

impl From<Vec<Passport>> for MemoryPassportRegister {
    fn from(value: Vec<Passport>) -> Self {
        let mut passports = HashMap::new();
        for val in value {
            passports.insert(val.id.clone(), val);
        }
        Self { passports }
    }
}

impl PassportRegister for MemoryPassportRegister {
    fn passport(&self, passport_id: &str) -> anyhow::Result<Option<Passport>> {
        Ok(self.passports.get(passport_id).map(|p| p.to_owned()))
    }
    fn set_passport(&mut self, passport: Passport) -> anyhow::Result<String> {
        let id = passport.id.clone();
        self.passports.insert(id.clone(), passport);
        Ok(id)
    }
    fn verify_credentials(
        &self,
        ticket: &Ticket,
    ) -> anyhow::Result<Option<Passport>> {
        let Some(passport) = self.passport(&ticket.id)? else {
            debug!("User with id {} not found.", ticket.id);
            return Ok(None);
        };
        if passport.verify_password(&ticket.secret)? {
            Ok(Some(passport))
        } else {
            Err(anyhow!("Invalid credentials."))
        }
    }
}
