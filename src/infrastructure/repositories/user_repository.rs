use crate::infrastructure::models::users::Person;
use crate::infrastructure::models::users::User;
use sqlx::PgPool;

pub async fn find_many(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!( User, r#"SELECT users.id, users.email, users.username, (persons.id,persons.first_name, persons.last_name) as "person: Person" FROM users INNER JOIN persons ON users.id_person = persons.id"#)
            .fetch_all(pool)
            .await
}

pub async fn find_one(pool: &PgPool, id_user: i32) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, r#"SELECT users.id, users.email, users.username, (persons.id,persons.first_name, persons.last_name) as "person: Person" FROM users INNER JOIN persons ON users.id_person = persons.id where users.id = $1"#,
            id_user
        )
        .fetch_one(pool)
        .await
}
