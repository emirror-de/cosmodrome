use serde::{
    Deserialize,
    Serialize,
};

/// Defines credentials for a simple login based on an `id` and a `secret`.
#[derive(Serialize, Deserialize, Debug)]
pub struct Ticket {
    /// The identification of the user, eg. a username.
    pub id: String,
    /// The secret of the user, eg. a password.
    pub secret: String,
}

impl Ticket {
    /// Creates a new ticket with the given id and secret.
    pub fn new(id: &str, secret: &str) -> Self {
        Self {
            id: id.to_string(),
            secret: secret.to_string(),
        }
    }
}
