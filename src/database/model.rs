use serde::Serialize;
use sqlx::{Error, FromRow, PgPool};
#[derive(Debug)]
pub struct ModelOrderRelation {
    pub model_id: String,
    pub order_hash: String,
    pub status: i32,
}

#[derive(Serialize, Debug, FromRow)]
pub struct Order {
    pub hash: i64,
    pub offerer: String,
    pub zone: i64,
    pub zone_hash: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub order_type: i32,
    pub total_original_consideration_items: i64,
    pub salt: String,
    pub counter: i64,
    pub conduit_key: i64,
    pub signature: String,
    pub cancelled: bool,
    pub finalized: bool,
    pub marked_invalid: bool,
}

// insert
impl ModelOrderRelation {
    // insert model order relation
    pub async fn insert_model_order_ralation(
        pool: &PgPool,
        model_id: String,
        order_hash: String,
        status: i32,
    ) -> Result<(), Error> {
        // 定义查询结果
        let quer_res = sqlx::query(
            "
            INSERT INTO model_order_relation (model_id, order_hash, status) VALUES ($1, $2, $3)
            ",
        )
        .bind(model_id)
        .bind(order_hash)
        .bind(status)
        .execute(pool)
        .await?;
        Ok(())
    }

    // update model order relation
    pub async fn update_model_order_ralation(
        pool: &PgPool,
        model_id: String,
        order_hash: String,
        status: i32,
    ) -> Result<(), Error> {
        // 定义查询结果
        let quer_res = sqlx::query(
            "
            UPDATE model_order_relation SET status = $1 WHERE model_id = $2 AND order_hash = $3
            ",
        )
        .bind(status)
        .bind(model_id)
        .bind(order_hash)
        .execute(pool)
        .await?;
        Ok(())
    }
    // select transaction by model id
    pub async fn select_transaction_by_model_id(
        pool: &PgPool,
        model_id: String,
    ) -> Result<Vec<Order>, Error> {
        // 定义查询结果
        let quer_res = sqlx::query_as::<_, Order>(
            "
            SELECT * from orders where hash in 
            (
                SELECT order_hash FROM model_order_relation WHERE model_id = 
            )
            ",
        )
        .bind(model_id)
        .fetch_all(pool)
        .await?;
        Ok(quer_res)
    }
}
