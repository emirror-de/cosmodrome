use serde::{
    Deserialize,
    Serialize,
};

/// Contains the credentials for a simple login.
#[derive(Serialize, Deserialize, Debug)]
pub struct Ticket {
    /// The identification of the user, eg. a username.
    pub id: String,
    /// The secret of the user, eg. a password.
    pub secret: String,
}
