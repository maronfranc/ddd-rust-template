use serde::{Deserialize, Serialize};

use crate::infrastructure::models::users::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct DtoUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub person: Option<DtoPerson>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtoPerson {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtoRegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub id_person: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub fn convert_user_into_dto(db_user: User) -> DtoUser {
    DtoUser {
        id: db_user.id,
        email: db_user.email,
        username: db_user.username,
        person: db_user.person.map(|person| DtoPerson {
            id: person.id,
            first_name: person.first_name,
            last_name: person.last_name,
        }),
    }
}
