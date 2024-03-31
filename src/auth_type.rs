//! Different auth types that are available.

/// The auth type to be used.
pub trait AuthType {}

/// Using the [Cookie] [AuthType] requires a [JwtCipher] in
/// [rocket]s global state.
#[derive(Debug)]
pub struct Cookie;

impl AuthType for Cookie {}
