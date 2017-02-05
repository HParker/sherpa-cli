use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use error::Error;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub github_handle: String,
    pub github_token: String,
    pub token: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    github_handle: &'a str,
    github_token: &'a str,
    token: &'a str,
}

pub fn find_or_create_user<'a>(conn: &SqliteConnection, github_handle: &'a str, github_token: &'a str, token: &'a str) -> Result<User, Error> {
    use schema::users::dsl::{github_handle as github_handle_filter, users};

    match users.filter(github_handle_filter.eq(github_handle)).first(conn) {
        Ok(user) => {
            user.github_token = github_token.to_owned();
            user.token = token.to_owned();
            user.save_changes(&conn)
        },
        Err(_error) => {
            let new_user = NewUser {
                github_handle: github_handle,
                github_token: github_token,
                token: token,
            };

            diesel::insert(&new_user).into(users).execute(conn)
        },
    }
}
