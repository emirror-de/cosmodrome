use cosmodrome::{
    auth_type::Bearer,
    boarding_pass::{
        payloads::JsonWebToken,
        BoardingPass,
    },
    ciphering::JwtCipher,
    gate::{
        Gate,
        JwtBearerGate,
    },
    passport::{
        Passport,
        PassportType,
    },
    passport_register::MemoryPassportRegister,
    storage::{
        Storage,
    },
    Ticket,
};
use rocket::{
    fs::NamedFile,
    get,
    http::{
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
) -> (Status, String) {
    let storage = Storage::new((), (), cipher.inner().to_owned());
    match JwtBearerGate::login(
        credentials.into_inner(),
        register.inner(),
        &storage,
    ) {
        Ok(token) => (Status::Ok, token),
        Err(e) => {
            log::error!("{e}");
            (Status::Unauthorized, String::new())
        }
    }
}

#[get("/private")]
async fn private(user: BoardingPass<JsonWebToken, Bearer>) -> String {
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
