use cosmodrome::{
    gate::{
        Gate,
        JwtCookieGate,
    },
    passport::{
        MemoryPassportRegister,
        Passport,
        PassportType,
    },
    BoardingPass,
    Cookie,
    CookieStorageOptions,
    JsonWebToken,
    JwtCipher,
    Storage,
    Ticket,
};
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

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("index.html").await.ok()
}

#[post("/login", format = "json", data = "<credentials>")]
async fn login(
    credentials: Json<Ticket>,
    register: &State<MemoryPassportRegister>,
    cipher: &State<JwtCipher>,
    cookies: &CookieJar<'_>,
) -> Status {
    let storage = Storage::new(
        cookies,
        CookieStorageOptions::default(),
        cipher.inner().to_owned(),
    );
    match JwtCookieGate::login(
        credentials.into_inner(),
        register.inner(),
        &storage,
    ) {
        Ok(_) => Status::Ok,
        Err(e) => {
            log::error!("{e}");
            Status::Unauthorized
        }
    }
}

#[get("/private")]
async fn private(user: BoardingPass<JsonWebToken, Cookie>) -> String {
    format!("{user:#?}")
}

#[launch]
fn simple_login() -> _ {
    // We need to have a global register where all users are stored.
    let register = MemoryPassportRegister::from(vec![Passport::new(
        "simple_user",
        "somepassword",
        &["simple_service"],
        PassportType::Admin,
    )
    .unwrap()]);
    let cipher = JwtCipher::random();

    rocket::build()
        .mount("/", routes![index, private, login])
        .manage(register)
        .manage(cipher)
}
