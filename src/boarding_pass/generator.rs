use crate::{
    gate::GateType,
    BoardingPassOld,
    Passport,
};
use anyhow::anyhow;
/// Responsible for generating the [BoardingPassOld] and its corresponding token.
pub trait BoardingPassGenerator<T: GateType> {
    /// Generates the [BoardingPass] based on the given [Passport] (usually returned by [SecurityCheck::verify]).
    fn generate_boarding_pass(
        &self,
        passport: &Passport,
    ) -> anyhow::Result<BoardingPassOld<T>> {
        BoardingPassOld::<T>::new(passport)
    }
}

impl<T: GateType> BoardingPassGenerator<T> for jsonwebtoken::Algorithm {
    fn generate_boarding_pass(
        &self,
        passport: &Passport,
    ) -> anyhow::Result<BoardingPassOld<T>> {
        Err(anyhow!("asdk"))
    }
}
