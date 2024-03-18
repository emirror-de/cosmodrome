/// Indicates the type of an account. Also known as a role.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(DbEnum))]
pub enum AccountType {
    Admin,
    Moderator,
    User,
}

/*
use {
    custom_derive::custom_derive,
    enum_derive::{EnumDisplay, EnumFromStr},
};
custom_derive! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, EnumFromStr, EnumDisplay)]
    #[cfg_attr(feature = "database", derive(DbEnum))]
    pub enum AccountType {
        Admin,
        Moderator,
        User,
    }
}
*/
