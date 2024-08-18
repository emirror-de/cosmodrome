//! Different auth methods, eg. `Cookie` or `Bearer`.

/// The auth type to be used.
pub trait AuthType {}

/// Using the [Cookie] [AuthType] requires a [JwtCipher](crate::ciphering::JwtCipher) in
/// [rocket]s global state.
#[derive(Debug)]
pub struct Cookie;

impl AuthType for Cookie {}

/// Using the [Bearer] [AuthType] requires a [JwtCipher](crate::ciphering::JwtCipher) in
/// [rocket]s global state.
#[derive(Debug)]
pub struct Bearer;

impl Bearer {
    /// Extracts that value from the given slice with respect to the given prefix.
    pub fn extract_value(
        authorization_header: &str,
        prefix: Option<String>,
    ) -> Option<String> {
        if !authorization_header.starts_with("Bearer ") {
            return None;
        }

        let Some(token) = authorization_header.strip_prefix("Bearer ") else {
            return None;
        };
        if let Some(p) = prefix {
            token.strip_prefix(&p).map(|t| t.to_string())
        } else {
            Some(token.to_string())
        }
    }
}

impl AuthType for Bearer {}
