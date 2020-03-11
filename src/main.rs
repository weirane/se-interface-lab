#[macro_use]
extern crate diesel;

mod errors;
mod schema;
mod util;

use errors::Errors;

use actix_web::{post, web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error as DError;
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Deserialize, Serialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Query {
    date: String,
    token: Uuid,
}

#[post("/user/signup")]
async fn signup(pool: web::Data<DbPool>, data: web::Json<Login>) -> Result<String, Errors> {
    eprintln!("{:#?}", data);
    let conn = pool.get().map_err(|_| Errors::DBConnError)?;

    // Add user to database
    schema::insert_new_user(&data.username, &data.password, &conn).map_err(|e| {
        use diesel::result::DatabaseErrorKind::UniqueViolation;
        if let DError::DatabaseError(UniqueViolation, _) = e {
            Errors::UserExists
        } else {
            e.into()
        }
    })?;
    let res = json!({ "success": true });
    Ok(res.to_string())
}

#[post("/user/signin")]
async fn signin(pool: web::Data<DbPool>, data: web::Json<Login>) -> Result<String, Errors> {
    eprintln!("{:#?}", data);
    let conn = pool.get().map_err(|_| Errors::DBConnError)?;

    // Check if it is a valid user
    let has_user = schema::valid_user(&data.username, &data.password, &conn)?;
    if !has_user {
        return Err(Errors::InvalidLogin);
    }

    // Generate a token
    let token = Uuid::new_v4();
    schema::add_token(&token, &conn)?;
    let res = json!({ "success": true, "token": token });
    Ok(res.to_string())
}

#[post("/date")]
async fn date(pool: web::Data<DbPool>, data: web::Json<Query>) -> Result<String, Errors> {
    eprintln!("{:#?}", data);
    let conn = pool.get().map_err(|_| Errors::DBConnError)?;

    // Check token
    if !schema::has_token(&data.token, &conn)? {
        return Err(Errors::InvalidToken);
    }

    // Parse date
    let ymd = util::parse_date(&data.date)?;
    let rslt = match ymd {
        (_, 1, 1) => "new year",
        (_, 10, 1) => "national day",
        (_, 12, 25) => "christmas",
        _ => "",
    };

    let ret = json!({ "success": true, "info": rslt });
    Ok(ret.to_string())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let connspec = std::env::var("DATABASE_URL").unwrap_or_else(|_| "users.db".to_string());
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8001";
    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .data(pool.clone())
            .service(signup)
            .service(signin)
            .service(date)
    })
    .bind(bind)?
    .run()
    .await
}
