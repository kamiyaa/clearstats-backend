use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::{DatabaseInteger, DatabaseResult};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: DatabaseInteger,
    pub title: String,
    pub description: String,
    pub upvotes: DatabaseInteger,
    pub downvotes: DatabaseInteger,
    pub question_count: DatabaseInteger,
    pub user_vote: Option<i16>,
    pub created_at: DatabaseInteger,
    pub updated_at: DatabaseInteger,
    pub posted_by_id: DatabaseInteger,
    pub posted_by_username: String,
    pub posted_by_created_at: DatabaseInteger,
}

#[derive(Clone, Debug)]
pub struct SqlQuery {
    pub user_id: Option<DatabaseInteger>,
    pub search: Option<String>,
    pub tag: Option<String>,
    pub page_size: DatabaseInteger,
    pub page_index: DatabaseInteger,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    params: &SqlQuery,
) -> DatabaseResult<Vec<SqlData>> {
    let pool = db_manager.get_database_pool();

    let mut next_param = 2usize; // $1 is user_id
    let search_query = match params.search.as_ref() {
        Some(_) => {
            let q = format!("AND (s.title LIKE ${} OR s.description LIKE ${})", next_param, next_param + 1);
            next_param += 2;
            q
        }
        None => String::new(),
    };
    let tag_query = match params.tag.as_ref() {
        Some(_) => {
            let q = format!("AND s.id IN (SELECT statistic_id FROM statistic_tag WHERE tag = ${})", next_param);
            next_param += 1;
            q
        }
        None => String::new(),
    };
    let limit_ph = format!("${}", next_param);
    let offset_ph = format!("${}", next_param + 1);

    let sql_query = format!(
        "SELECT
            s.id,
            s.title,
            s.description,
            s.upvotes,
            s.downvotes,
            s.question_count,
            s.created_at,
            s.updated_at,
            s_vote.vote AS user_vote,
            user_profile.user_id AS posted_by_id,
            user_profile.username AS posted_by_username,
            user_profile.created_at AS posted_by_created_at
        FROM
            statistic s
        INNER JOIN
            user_profile
        ON
            s.posted_by_user_id = user_profile.user_id
        LEFT JOIN
            statistic_vote s_vote
        ON
            s_vote.statistic_id = s.id
        AND
            s_vote.user_id = $1
            {search_query}
            {tag_query}
        ORDER BY s.created_at DESC
        LIMIT {limit_ph} OFFSET {offset_ph}"
    );

    let mut sql_res = sqlx::query_as(&sql_query).bind(params.user_id);

    if let Some(v) = params.search.as_ref() {
        let search_pattern = format!("%{}%", v);
        sql_res = sql_res.bind(search_pattern.clone()).bind(search_pattern);
    }
    if let Some(v) = params.tag.as_ref() {
        sql_res = sql_res.bind(v);
    }
    let sql_res = sql_res
        .bind(params.page_size)
        .bind(params.page_index * params.page_size)
        .fetch_all(pool)
        .await?;
    return Ok(sql_res);
}
