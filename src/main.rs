#[macro_use]
extern crate diesel;

mod errors;
mod schema;
mod util;

use errors::Errors;

use actix_web::{middleware, post, web, App, HttpResponse, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error as DError;
use diesel::SqliteConnection;
use log::{debug, info};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
type SMap = HashMap<String, String>;

#[derive(Debug)]
struct Login {
    username: String,
    password: String,
}

#[derive(Debug)]
struct Query {
    date: String,
    token: Uuid,
}

#[post("/user/signup")]
async fn signup(pool: web::Data<DbPool>, data: web::Form<SMap>) -> Result<HttpResponse, Errors> {
    debug!("{:?}", data);
    let data = Login {
        username: data.get("username").ok_or(Errors::BadRequest)?.to_string(),
        password: data.get("password").ok_or(Errors::BadRequest)?.to_string(),
    };

    let conn = pool.get()?;

    // Add user to database
    schema::insert_new_user(&data.username, &data.password, &conn).map_err(|e| {
        use diesel::result::DatabaseErrorKind::UniqueViolation;
        if let DError::DatabaseError(UniqueViolation, _) = e {
            Errors::UserExists(data.username.to_string())
        } else {
            e.into()
        }
    })?;
    let res = json!({ "success": true });
    Ok(HttpResponse::Ok().json(res))
}

#[post("/user/signin")]
async fn signin(pool: web::Data<DbPool>, data: web::Form<SMap>) -> Result<HttpResponse, Errors> {
    debug!("{:?}", data);
    let data = Login {
        username: data.get("username").ok_or(Errors::BadRequest)?.to_string(),
        password: data.get("password").ok_or(Errors::BadRequest)?.to_string(),
    };

    let conn = pool.get()?;

    // Check if it is a valid user
    let has_user = schema::valid_user(&data.username, &data.password, &conn)?;
    if !has_user {
        return Err(Errors::InvalidLogin(
            data.username.clone(),
            data.password.clone(),
        ));
    }

    // Generate a token
    let token = Uuid::new_v4();
    schema::add_token(&token, &conn)?;
    let res = json!({ "success": true, "token": token });
    Ok(HttpResponse::Ok().json(res))
}

#[post("/date")]
async fn date(pool: web::Data<DbPool>, data: web::Form<SMap>) -> Result<HttpResponse, Errors> {
    debug!("{:?}", data);

    let data = Query {
        date: data.get("date").ok_or(Errors::BadRequest)?.to_string(),
        token: data
            .get("token")
            .and_then(|s| s.parse().ok())
            .ok_or(Errors::BadRequest)?,
    };

    let conn = pool.get()?;

    // Check token
    if !schema::has_token(&data.token, &conn)? {
        return Err(Errors::InvalidToken(data.token));
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
    Ok(HttpResponse::Ok().json(ret))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    debug!("Current directory: {}", std::env::current_dir()?.display());

    let connspec = std::env::var("DATABASE_URL").unwrap_or_else(|_| "users.db".to_string());
    info!("Using database {}", connspec);
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8001";
    info!("Using address {}", bind);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::FormConfig::default().limit(4096))
            .data(pool.clone())
            .service(signup)
            .service(signin)
            .service(date)
    })
    .bind(bind)?
    .run()
    .await
}
