use crate::database::DatabasePool;

pub trait DatabaseManagerTrait {
    fn get_database_pool(&self) -> &DatabasePool;
}

#[derive(Clone, Debug)]
pub struct DatabaseManager {
    pub pool: DatabasePool,
}

impl DatabaseManager {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }
}

impl DatabaseManagerTrait for DatabaseManager {
    fn get_database_pool(&self) -> &DatabasePool {
        &self.pool
    }
}
