use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::{TABLE_USER_CREDENTIAL, TABLE_USER_PROFILE};
use shared_lib::database::{DatabaseInteger, DatabaseResult};

#[derive(Clone, Debug)]
pub struct SqlData {
    pub user_id: DatabaseInteger,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub async fn run_query(db_manager: &DatabaseManager, data: &SqlData) -> DatabaseResult<u64> {
    let pool = db_manager.get_database_pool();

    let mut rows_affected = 0;
    let mut tx = pool.begin().await?;
    if let Some(email) = &data.email {
        let sql_query = format!(
            "
            UPDATE
                {TABLE_USER_CREDENTIAL}
            SET
                email = ?
            WHERE
                id = ?
            ;"
        );
        let sql_res = sqlx::query(&sql_query)
            .bind(email)
            .bind(data.user_id)
            .execute(&mut *tx)
            .await?;
        rows_affected += sql_res.rows_affected();
        if sql_res.rows_affected() == 0 {
            tx.rollback().await?;
            return Err(sqlx::Error::RowNotFound);
        }
    }

    if data.first_name.is_some() && data.last_name.is_some() {
        let sql_query = format!(
            "
UPDATE
    {TABLE_USER_PROFILE}
SET
    first_name = ?,
    last_name = ?
WHERE
    user_id = ?
;"
        );
        let sql_res = sqlx::query(&sql_query)
            .bind(&data.first_name)
            .bind(&data.last_name)
            .bind(data.user_id)
            .execute(&mut *tx)
            .await?;
        rows_affected += sql_res.rows_affected();

        if sql_res.rows_affected() == 0 {
            tx.rollback().await?;
            return Err(sqlx::Error::RowNotFound);
        }
    }

    tx.commit().await?;
    Ok(rows_affected)
}
