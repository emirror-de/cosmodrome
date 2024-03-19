use rocket::serde::{
    Deserialize,
    Serialize,
};

/// Indicates the access level of an account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum AccountType {
    Admin,
    Moderator,
    User,
    #[serde(untagged)]
    Custom(String),
}
