use crate::application::dtos::user_dto::DtoUser;
use crate::infrastructure::models::users::Id;
use crate::infrastructure::models::users::Person;
use crate::infrastructure::models::users::User;
use sqlx::PgPool;

pub async fn create_one(pool: &PgPool, user: DtoUser) -> Result<Id, sqlx::Error> {
    let id_person = user.person.map(|person| person.id);
    sqlx::query_as!(
        Id,
        r#"INSERT INTO users(id,email,username,"password",id_person) VALUES($1,$2,$3,$4,$5) RETURNING id"#,
        user.id, user.email, user.username, "TODO:password", id_person,
    )
    .fetch_one(pool).await
}

pub async fn find_many(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!( User, r#"SELECT users.id, users.email, users.username, (persons.id,persons.first_name, persons.last_name) as "person: Person" FROM users INNER JOIN persons ON users.id_person = persons.id"#)
            .fetch_all(pool)
            .await
}

pub async fn find_by_id(pool: &PgPool, id_user: i32) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, r#"SELECT users.id, users.email, users.username, (persons.id,persons.first_name, persons.last_name) as "person: Person" FROM users INNER JOIN persons ON users.id_person = persons.id where users.id = $1"#,
            id_user
        )
        .fetch_one(pool)
        .await
}
