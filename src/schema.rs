use diesel::prelude::*;
use uuid::Uuid;

use diesel::result::Error as DError;

mod schema {
    table! {
        users(username) {
            username -> Text,
            password -> Text,
        }
    }

    table! {
        tokens(uuid) {
            uuid -> Text,
        }
    }
}

use self::schema::{tokens, users};

#[derive(Debug, Clone, serde::Serialize, Queryable, Insertable)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Serialize, Queryable, Insertable)]
pub struct Token {
    pub uuid: String,
}

pub fn insert_new_user(name: &str, pass: &str, conn: &SqliteConnection) -> Result<(), DError> {
    use self::users::dsl::*;

    let new_user = User {
        username: name.to_string(),
        password: pass.to_string(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(())
}

pub fn valid_user(name: &str, pass: &str, conn: &SqliteConnection) -> Result<bool, DError> {
    use self::users::dsl::*;

    let rslt = users
        .filter(username.eq(name))
        .filter(password.eq(pass))
        .first::<User>(conn);
    match rslt {
        Ok(_) => Ok(true),
        Err(diesel::NotFound) => Ok(false),
        Err(e) => Err(e),
    }
}

pub fn add_token(tok: &Uuid, conn: &SqliteConnection) -> Result<(), DError> {
    use self::tokens::dsl::*;
    let new_tok = Token {
        uuid: tok.to_string(),
    };
    diesel::insert_into(tokens).values(&new_tok).execute(conn)?;
    Ok(())
}

pub fn has_token(tok: &Uuid, conn: &SqliteConnection) -> Result<bool, DError> {
    use self::tokens::dsl::*;
    let rslt = tokens
        .filter(uuid.eq(&tok.to_string()))
        .first::<Token>(conn);
    match rslt {
        Ok(_) => Ok(true),
        Err(diesel::NotFound) => Ok(false),
        Err(e) => Err(e),
    }
}
