use crate::application::dtos::user_dto::DtoRegisterUser;
use crate::infrastructure::models::users::Id;
use crate::infrastructure::models::users::Person;
use crate::infrastructure::models::users::User;
use sqlx::PgPool;

pub async fn create_one(pool: &PgPool, user: DtoRegisterUser) -> Result<Id, sqlx::Error> {
    sqlx::query_as!(
        Id,
        r#"INSERT INTO users(id,email,username,"password",id_person)
        VALUES(default,$1,$2,$3,$4) RETURNING id"#,
        user.email,
        user.username,
        user.password,
        user.id_person
    )
    .fetch_one(pool)
    .await
}

pub async fn find_many(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
    // CASE condition used as a workaround if joined values is null.
    // Remove is if this issues is solved: https://github.com/launchbadge/sqlx/issues/367
        r#"SELECT
    users.id, users.email, users.username,
(CASE WHEN persons.id IS NOT NULL then 
    (persons.id,persons.first_name,persons.last_name) END) AS "person: Person"
FROM users LEFT JOIN persons ON users.id_person = persons.id"#
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id_user: i32) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
    // CASE condition used as a workaround if joined values is null.
    // Remove is if this issues is solved: https://github.com/launchbadge/sqlx/issues/367
        r#"SELECT
    users.id, users.email, users.username,
(CASE WHEN persons.id IS NOT NULL then 
    (persons.id,persons.first_name,persons.last_name) END) AS "person: Person"
FROM users LEFT JOIN persons ON users.id_person = persons.id 
    WHERE users.id = $1"#,
        id_user
    )
    .fetch_one(pool)
    .await
}
