use rocket::serde::{
    Deserialize,
    Serialize,
};

/// Indicates the access level of an account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum PassportType {
    /// The person having this type is considered an Administrator.
    Admin,
    /// The person having this type is considered a Moderator.
    Moderator,
    /// The person having this type is considered a User. This is usually the most used type of passport.
    User,
    /// A custom passport type.
    #[serde(untagged)]
    Custom(String),
}
