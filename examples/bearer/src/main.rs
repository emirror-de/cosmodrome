use cosmodrome::{
    gate::{
        memory::{
            MemoryGate,
            MemoryGateOptions,
        },
        Bearer,
        Cookie,
        Gate,
    },
    BoardingPass,
    Passport,
    PassportType,
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
    gate: &State<MemoryGate>,
) -> (Status, String) {
    match gate.login(credentials.into_inner(), ()) {
        Ok(b) => (Status::Ok, b),
        Err(e) => {
            log::error!("{e}");
            (Status::Unauthorized, String::new())
        }
    }
}

#[get("/private")]
async fn private(user: BoardingPass<Bearer>) -> String {
    format!("{user:#?}")
}

#[launch]
fn simple_login() -> _ {
    // We can customize the settings of the gate before launch.
    let gate_settings = MemoryGateOptions::default();
    let gate = MemoryGate::from((
        vec![Passport::new(
            "simple_user",
            "somepassword",
            &["simple_service"],
            PassportType::Admin,
        )
        .unwrap()],
        gate_settings,
    ));

    rocket::build()
        .mount("/", routes![index, private, login])
        .manage(gate)
}
