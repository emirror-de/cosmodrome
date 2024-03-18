use crate::{
    account_claims::AccountClaims,
    AuthSettings,
};
use anyhow::anyhow;
use rocket::{
    http::Status,
    request::{
        FromRequest,
        Outcome,
        Request,
    },
};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AccountClaims {
    type Error = anyhow::Error;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        let Some(settings) = request.rocket().state::<AuthSettings>() else {
            return Outcome::Forward(Status::InternalServerError);
        };
        let cookies = request.cookies();

        let auth = cookies.get(settings.cookie_name());
        let Some(auth) = &auth else {
            return Outcome::Error((
                Status::Unauthorized,
                anyhow!("No auth cookie available."),
            ));
        };
        let user = AccountClaims::decode(
            auth.value(),
            settings.authentication_secret(),
        );
        match user {
            Err(e) => {
                return Outcome::Error((Status::Unauthorized, anyhow!("{e}")));
            }
            Ok(u) => Outcome::Success(u),
        }
    }
}
