#[derive(sqlx::FromRow, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub person: Option<Person>,
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct InsertUser {
    pub username: String,
    pub email: String,
}

#[derive(sqlx::FromRow, sqlx::Decode, Clone, Debug)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Id {
    pub id: i32,
}
