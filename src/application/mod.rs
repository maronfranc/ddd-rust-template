pub mod controllers;
pub mod dtos;

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct State {
    pub pool: PgPool,
}
