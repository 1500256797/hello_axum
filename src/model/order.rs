use serde::Serialize;
use sqlx::{pool, postgres::PgRow, Error, FromRow, PgPool, Postgres, Row};

#[derive(Debug, FromRow, Serialize)]
pub struct Orders {
    order_type: i32,
    counter: i64,
}

impl Orders {
    pub async fn get_orders_count(pool: &PgPool) -> Result<Vec<Orders>, Error> {
        // 定义查询结果
        let quer_res = sqlx::query_as::<Postgres, Orders>(
            "
            SELECT order_type, count(*) as counter FROM orders group by order_type
            ",
        )
        .fetch_all(pool)
        .await?;
        Ok(quer_res)
    }

    pub async fn get_orders_count_v2(pool: &PgPool) -> Result<Vec<Orders>, Error> {
        // 定义查询结果
        let quer_res = sqlx::query::<Postgres>(
            "
            SELECT order_type, count(*) as counter FROM orders group by order_type
            ",
        )
        .map(|row: PgRow| Orders {
            order_type: row.get(0),
            counter: row.get(1),
        })
        .fetch_all(pool)
        .await?;
        Ok(quer_res)
    }
}

#[derive(Serialize, Debug, FromRow)]
pub struct OrderInput {
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

impl OrderInput {
    // insert order
    pub async fn insert_order(
        pool: &PgPool,
        order_input: &OrderInput,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        // insert
        let insert_res: sqlx::postgres::PgQueryResult = sqlx::query::<Postgres>(
            r#"
            insert INTO orders (hash,offerer,zone,zone_hash,start_time,end_time,order_type,total_original_consideration_items,salt ,counter,conduit_key,signature,cancelled,finalized,marked_invalid)
VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
            "#).bind(order_input.hash)
            .bind(order_input.offerer.as_str())
            .bind(order_input.zone)
            .bind(order_input.zone_hash)
            .bind(order_input.start_time)
            .bind(order_input.end_time)
            .bind(order_input.order_type)
            .bind(order_input.total_original_consideration_items)
            .bind(order_input.salt.as_str())
            .bind(order_input.counter)
            .bind(order_input.conduit_key)
            .bind(order_input.signature.as_str())
            .bind(order_input.cancelled)
            .bind(order_input.finalized)
            .bind(order_input.marked_invalid)
            .execute(pool).await?;
        Ok(insert_res)
    }
    // update order
    pub async fn update_orders(
        pool: &PgPool,
        order_input: &OrderInput,
    ) -> Result<u64, sqlx::Error> {
        // update sql
        let update_res = sqlx::query::<Postgres>(
            r#"
                UPDATE orders SET start_time = $1, end_time = $2 WHERE hash = $3
            "#,
        )
        .bind(order_input.start_time)
        .bind(order_input.end_time)
        .bind("1234568")
        .execute(pool)
        .await?;

        // return
        Ok(update_res.rows_affected())
    }
}
