use sqlx::prelude::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlCount {
    pub count: i64,
}
