//! Data structures that deal with an account that is required to have access to a service.
pub(crate) mod account_type;

pub use account_type::AccountType;

use {
    crate::macros::*,
    argon2::{self, Config},
    chrono::prelude::*,
    rand::{distributions::Alphanumeric, thread_rng, Rng},
};

#[cfg(feature = "database")]
use {crate::models::base::Person, database::schema::accounts};

/// Defines a user account of a service. Can be used in connection with ```Person```.
#[model(table_name = "accounts")]
#[cfg_attr(feature = "database", belongs_to(Person, foreign_key = "person_id"))]
pub struct Account {
    pub id: i32,
    pub person_id: Option<i32>,
    pub username: String,
    pub nickname: String,
    pub email: String,
    password: String,
    service: String,
    pub account_type: AccountType,
    pub active: bool,
    pub confirmed: bool,
    pub expires_at: NaiveDateTime,
    created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Account {
    pub fn new(
        person_id: Option<i32>,
        username: String,
        nickname: Option<String>,
        email: String,
        password: String,
        service: String,
        account_type: AccountType,
    ) -> Self {
        let nickname = nickname.unwrap_or(username.clone());
        Self {
            id: 0,
            person_id: person_id,
            username,
            nickname,
            email,
            password: Self::hash_password(password),
            service: service,
            account_type,
            active: true,     // always activate
            confirmed: false, // always require user to confirm it
            expires_at: chrono::Utc::now().naive_utc(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    /// Returns the service this account belongs to.
    pub fn get_service(&self) -> String {
        self.service.clone()
    }

    /// Saves the ```new_password``` to the struct after verifying the ```old_password```.
    /// Does NOT automatically call the ```update``` function to update the database.
    pub fn change_password(&mut self, old_password: String, new_password: String) -> bool {
        if self.verify_password(old_password) {
            self.password = Self::hash_password(new_password);
            true
        } else {
            false
        }
    }

    /// Checks if the given password is correct.
    pub fn verify_password(&self, password: String) -> bool {
        argon2::verify_encoded(&self.password, password.as_bytes()).unwrap()
    }

    /// Hashes the password using ```argon2::hash_encoded```.
    fn hash_password(password: String) -> String {
        let salt: String = thread_rng().sample_iter(&Alphanumeric).take(256).collect();
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
    }
}

#[cfg(feature = "database")]
pub(crate) mod database {
    use {
        crate::database::DatabaseMigration,
        barrel::{backend::SqlGenerator, types, Migration},
    };

    impl super::Account {
        /// Name of the table where this struct is stored in the database.
        pub const TABLE_NAME: &'static str = "accounts";
    }

    pub mod schema {
        table! {
            accounts (id) {
                id -> Int4,
                person_id -> Nullable<Int4>,
                username -> Varchar,
                nickname -> Varchar,
                email -> Varchar,
                password -> Varchar,
                service -> Varchar,
                account_type -> crate::models::account::account_type::AccountTypeMapping,
                active -> Bool,
                confirmed -> Bool,
                expires_at -> Timestamp,
                created_at -> Timestamp,
                updated_at -> Timestamp,
            }
        }
    }

    impl DatabaseMigration for super::Account {
        fn migration_up<T: SqlGenerator>() -> String {
            let mut m = Migration::new();
            m.create_table_if_not_exists(Self::TABLE_NAME, |t| {
                t.add_column("id", types::primary());
                t.add_column(
                    "person_id",
                    types::foreign(
                        crate::models::base::Person::TABLE_NAME,
                        vec!["id"],
                        types::ReferentialAction::Unset,
                        types::ReferentialAction::Cascade,
                    )
                    .nullable(true),
                );
                t.add_column("username", types::text().nullable(true));
                t.add_column("nickname", types::text().nullable(true));
                t.add_column("email", types::varchar(255).unique(true));
                t.add_column("password", types::text().nullable(true));
                t.add_column("service", types::date().nullable(true));
                t.add_column("account_type", types::text());
                t.add_column("active", types::boolean());
                t.add_column("confirmed", types::boolean());
                t.add_column("expires_at", types::datetime());
                t.add_column("updated_at", types::datetime());
                t.add_column("created_at", types::datetime());
            });
            m.make::<T>()
        }

        fn migration_down<T: SqlGenerator>() -> String {
            let mut m = Migration::new();
            m.drop_table_if_exists(Self::TABLE_NAME);
            m.make::<T>()
        }
    }

    impl crate::database::DatabaseBeforeUpdate for super::Account {
        fn on_before_update(&mut self) {
            self.updated_at = chrono::Utc::now().naive_utc();
        }
    }
}
