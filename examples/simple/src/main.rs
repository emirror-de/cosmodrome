use rocket::{
    fs::NamedFile,
    get,
    http::{
        CookieJar,
        Status,
    },
    launch,
    post,
    routes,
    serde::json::Json,
    State,
};
use rocket_webauth::{
    Account,
    AccountClaims,
    AccountCredentials,
    AccountProvider,
    AccountType,
    AuthSettings,
    MemoryAccountProvider,
};

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("index.html").await.ok()
}

#[post("/login", format = "json", data = "<credentials>")]
async fn login(
    credentials: Json<AccountCredentials>,
    account_provider: &State<MemoryAccountProvider>,
    auth_settings: &State<AuthSettings>,
    cookies: &CookieJar<'_>,
) -> Status {
    match account_provider.login(
        credentials.into_inner(),
        auth_settings.inner(),
        cookies,
    ) {
        Ok(_) => Status::Ok,
        Err(e) => {
            log::error!("{e}");
            Status::Unauthorized
        }
    }
}

#[get("/private")]
async fn private(user: AccountClaims) -> String {
    format!("{user:#?}")
}

#[launch]
fn simple_login() -> _ {
    // We create a global state containing information with the cookie name and encryption secret.
    let auth_settings = AuthSettings::new_with_random_secret("rocket_webauth");
    let auth_provider = MemoryAccountProvider::from(vec![Account::new(
        "simple_user",
        "somepassword",
        "simple_service",
        AccountType::Admin,
    )
    .unwrap()]);

    rocket::build()
        .mount("/", routes![index, private, login])
        .manage(auth_settings)
        .manage(auth_provider)
}
