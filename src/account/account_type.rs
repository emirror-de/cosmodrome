use rocket::serde::{
    Deserialize,
    Serialize,
};

/// Indicates the type of an account. Sometimes known as a role.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum AccountType {
    Admin,
    Moderator,
    User,
    #[serde(untagged)]
    Custom(String),
}
