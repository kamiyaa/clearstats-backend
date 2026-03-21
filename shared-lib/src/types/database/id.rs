use sqlx::prelude::FromRow;

use crate::database::DatabaseInteger;

#[derive(Clone, Debug, FromRow)]
pub struct SqlId {
    pub id: DatabaseInteger,
}
