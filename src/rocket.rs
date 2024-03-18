use {
    crate::account_claims::AccountClaims,
    rocket::{
        request::{self, FromRequest, Request},
        Outcome,
    },
};

impl<'a, 'r> FromRequest<'a, 'r> for AccountClaims {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let secret = match crate::get_authentication_secret() {
            Err(_) => {
                return Outcome::Forward(());
                //return Outcome::Failure(
                //    (Status::Unauthorized, "Internal server error!".to_string())
                //    );
            }
            Ok(v) => v,
        };
        let auth_cookie_name = match crate::get_cookie_name() {
            Err(_) => {
                return Outcome::Forward(());
                //return Outcome::Failure(
                //    (Status::Unauthorized, "Internal server error!".to_string())
                //    );
            }
            Ok(v) => v,
        };

        let cookies = request.cookies();

        let auth = cookies.get(&auth_cookie_name);
        if let None = &auth {
            return Outcome::Forward(());
            //return Outcome::Failure(
            //    (Status::Unauthorized, "Cookie not found!".to_string())
            //    );
        }
        let user = AccountClaims::decode(auth.unwrap().value(), &secret);
        if let Err(_) = &user {
            return Outcome::Forward(());
            //return Outcome::Failure(
            //    (Status::Unauthorized, msg.to_string())
            //    );
        }
        Outcome::Success(user.unwrap())
    }
}
