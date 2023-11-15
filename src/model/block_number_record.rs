use crate::DBType;
use sqlx::Error;

use crate::DBPool;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BlockNumberRecord {
    pub id: i32,
    pub chain_type: String,
    pub wss_url: String,
    pub https_url: String,
    pub contract_addr: String,
    pub event_type: String,
    pub event_signature: String,
    pub from_block: i32,
    pub last_block: i32,
    pub sechdule_time: i32,
    pub space_block: i32,
    pub desc: String,
}

// get last block number by event_type
impl BlockNumberRecord {
    pub async fn get_last_block_number_by_event_type(
        pool: &DBPool,
        event_type: String,
        chain_type: String,
    ) -> Result<i32, Error> {
        // get latest block number by event_type sql
        let query_res = sqlx::query_as::<_, BlockNumberRecord>(
            "
            SELECT * FROM block_number_record
            WHERE event_type = $1 AND chain_type = $2
            ",
        )
        .bind(event_type)
        .bind(chain_type)
        .fetch_one(pool)
        .await?;
        Ok(query_res.last_block)
    }

    // get block records by chain_type
    pub async fn get_block_records_by_chain_type(
        pool: &DBPool,
        chain_type: String,
    ) -> Result<Vec<BlockNumberRecord>, Error> {
        // get block records by chain_type sql
        let query_res = sqlx::query_as::<_, BlockNumberRecord>(
            "
            SELECT * FROM block_number_record
            WHERE chain_type = $1
            ",
        )
        .bind(chain_type)
        .fetch_all(pool)
        .await?;
        Ok(query_res)
    }

    pub async fn get_block_number_by_event_type(
        pool: &DBPool,
        event_type: String,
        chain_type: String,
    ) -> Result<BlockNumberRecord, Error> {
        // get latest block number by event_type sql
        let query_res = sqlx::query_as::<_, BlockNumberRecord>(
            "
            SELECT * FROM block_number_record
            WHERE event_type = $1 AND chain_type = $2
            ",
        )
        .bind(event_type)
        .bind(chain_type)
        .fetch_one(pool)
        .await?;
        Ok(query_res)
    }

    // upsert last block number by event_type
    pub async fn upsert_last_block_number_chain_event(
        transaction: &mut sqlx::Transaction<'_, DBType>,
        event_type: String,
        chain_type: String,
        last_block: i32,
    ) -> Result<u64, Error> {
        // upsert last block number by event_type sql
        let query_res = sqlx::query(
            "
            INSERT INTO block_number_record (event_type, chain_type, last_block)
            VALUES ($1, $2, $3)
            ON CONFLICT (event_type, chain_type)
            DO UPDATE SET last_block = EXCLUDED.last_block;
            ",
        )
        .bind(event_type)
        .bind(chain_type)
        .bind(last_block)
        .execute(&mut **transaction)
        .await?;
        Ok(query_res.rows_affected())
    }
}
