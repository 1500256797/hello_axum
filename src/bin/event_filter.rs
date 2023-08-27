use dotenv::dotenv;
use ethers::{
    core::types::{Address, Filter, H256, U256},
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    // load .env
    dotenv().ok();
    // get database url
    let wss_url = env::var("WSS_URL").expect("WSS_URL must be set");
    let lucky_draw = env::var("LUCKY_DRAW").expect("LUCKY_DRAW must be set");
    let from_block = env::var("FROM_BLOCK").expect("FROM_BLOCK must be set");
    // new db pool
    // let db_pool = get_connection_pool().await?;
    let provider = Provider::<Ws>::connect(wss_url).await?;
    let client = Arc::new(provider);
    let filter = Filter::new()
        .address(lucky_draw.parse::<Address>()?)
        .event("JoinActivity(address,uint256,bool)")
        .from_block(from_block.parse::<i32>()?);
    // let logs = client.get_logs(&filter).await.unwrap();
    // subscribe to events
    let mut stream = client.subscribe_logs(&filter).await?;
    while let Some(log) = stream.next().await {
        let user_address = Address::from(log.topics[1]);
        let activity_id = U256::from_big_endian(log.topics[2].as_bytes());
        let is_winner = U256::from_big_endian(log.topics[3].as_bytes());
        let transaction_hash: H256 = log.transaction_hash.unwrap();
        println!(
                "transaction_hash={transaction_hash}, user_address = {user_address}, activity_id = {activity_id}, is_winner = {is_winner}"
            );
        // if is_winner == 0
        // if is_winner.is_zero() {
        //     // upsert user points
        //     let _res = upsert_user_points_handler(
        //         db_pool.clone(),
        //         format!("{:?}", transaction_hash),
        //         format!("{:?}", user_address),
        //         activity_id.as_u64() as i64,
        //         is_winner.as_u64() as i64,
        //     )
        //     .await
        //     .unwrap();
        // }
    }
    Ok(())
}

// async fn get_connection_pool() -> Result<DBPool, sqlx::Error> {
//     // load .env
//     dotenv().ok();
//     // get database url
//     let database_url_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let pool = DBPoolOptions::new()
//         .idle_timeout(std::time::Duration::from_secs(2))
//         .max_connections(100)
//         .connect(&database_url_str)
//         .await?;
//     Ok(pool)
// }

// async fn upsert_user_points_handler(
//     db_pool: DBPool,
//     transaction_hash: String,
//     user_address: String,
//     activity_id: i64,
//     is_winner: i64,
// ) -> Result<()> {
//     println!(
//         "transaction_hash={transaction_hash}, user_address = {user_address}, activity_id = {activity_id}, is_winner = {is_winner}"
//     );
//     // upsert user points sql
//     let _res = UserPointsHistory::upsert_user_points_history(
//         &db_pool,
//         user_address,
//         transaction_hash,
//         10,
//         0,
//         "quickdraw".to_string(),
//         chrono::Utc::now(),
//         chrono::Utc::now(),
//     )
//     .await?;
//     Ok(())
// }
