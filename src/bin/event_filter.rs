use dotenv::dotenv;
use ethers::abi::RawLog;
use ethers::providers::Ws;
use ethers::types::{H256, U64};
use ethers::{
    core::types::{Address, Filter},
    providers::{Middleware, Provider},
};
use eyre::Result;
use hello_axum::model::block_number_record::BlockNumberRecord;
use hello_axum::{DBPool, DBPoolOptions};
use std::num::ParseIntError;
use std::time::Duration;
use std::{env, sync::Arc};
use tokio::time::interval;
#[tokio::main]
async fn main() -> Result<()> {
    let chain_type: &str = "bsc";
    // new db pool
    let db_pool = get_connection_pool().await?;
    let block_records = get_block_records_by_chain_type(db_pool.clone(), chain_type.to_string())
        .await
        .unwrap();
    // store all threads handles
    let mut handles = Vec::new();

    // concurrent run jobs
    for block_record in block_records.iter() {
        let block_record = block_record.clone();
        let handle = tokio::spawn(async move {
            match block_record.event_type.as_str() {
                "JoinActivity" => {
                    println!("JoinActivity job start");
                    join_activity_event_filter_task(
                        block_record.event_type,
                        block_record.chain_type,
                    )
                    .await
                    .unwrap();
                }
                "GalleryChanged" => {
                    println!("GalleryChanged job start");
                    create_space_event_filter_task(
                        block_record.event_type,
                        block_record.chain_type,
                    )
                    .await
                    .unwrap();
                }
                "WinnerErc20Amount" => {
                    println!("WinnerErc20Amount job start");
                    winner_prize_event_filter_task(
                        block_record.event_type,
                        block_record.chain_type,
                    )
                    .await
                    .unwrap();
                }
                _ => {}
            }
        });
        handles.push(handle);
    }
    // wait all jobs done
    for handle in handles {
        handle.await?;
    }
    Ok(())
}
async fn winner_prize_event_filter_task(event_type: String, chain_type: String) -> Result<()> {
    let db_pool = get_connection_pool().await?;
    let block_record = get_block_number_by_event_type(
        db_pool.clone(),
        event_type.to_string(),
        chain_type.to_string(),
    )
    .await
    .unwrap();
    // new schedule task
    let mut interval = interval(Duration::from_secs(block_record.sechdule_time as u64));
    loop {
        interval.tick().await;
        let block_record = get_block_number_by_event_type(
            db_pool.clone(),
            event_type.to_string(),
            chain_type.to_string(),
        )
        .await
        .unwrap();
        let provider = Provider::<Ws>::connect(block_record.wss_url).await?;
        let space_block = block_record.space_block;
        let to_block = block_record.last_block + space_block;
        let logs = fetch_log(
            provider.clone(),
            block_record.contract_addr.clone(),
            block_record.event_signature.clone(),
            block_record.last_block,
            to_block,
        )
        .await?;
        let current_block_number = provider.get_block_number().await?;
        // decode logs
        let decoded_logs: Vec<ethers::abi::Log> =
            parse_event_vec_log(logs.clone(), &block_record.event_signature).unwrap();
        // parse log
        let mut txn = db_pool.begin().await?;
        for (index, _value) in decoded_logs.iter().enumerate() {
            let block_number = logs[index].block_number.unwrap().as_u64() as i32;
            let user_address = decoded_logs[index].params[0].value.clone().to_string();
            // user_address to Address
            let user_address = Address::from(user_address.parse::<Address>()?);
            let user_address = format!("{:?}", user_address);
            let activity_id = decoded_logs[index].params[1].value.clone().to_string();
            // hex 16 to 10
            let activity_id = hex_to_decimal(activity_id.as_str()).unwrap();
            let prize_amount = decoded_logs[index].params[2].value.clone().to_string();
            // hex 16 to 10
            let prize_amount = hex_to_decimal(prize_amount.as_str()).unwrap();
            let prize_amount = ethers::utils::format_ether(prize_amount);
            // fix point 2 digits
            let prize_amount = format!("{:.4}", prize_amount);
            let transaction: H256 = logs[index].transaction_hash.unwrap();
            // get block time
            // let block_time = get_block_time(U64([block_number as u64])).await;
            // if block_time is None, skip this loop
            // if block_time.is_none() {
            //     continue;
            // }
            // let block_time = block_time.unwrap();
            // upsert block number record
            println!("chain_type = {:?} transaction = {:?} block_number = {:?} block_time = {:?} user_address = {:?} activity_id = {:?} prize_amount = {:?}", chain_type.clone(), format!("{:?}", transaction), block_number, 0, user_address.clone(), activity_id as i64, prize_amount);
        }
        // get min block number
        let min_block_number = min_block_number(current_block_number, to_block);
        // upsert block number record
        let _result = BlockNumberRecord::upsert_last_block_number_chain_event(
            &mut txn,
            event_type.clone(),
            chain_type.clone(),
            min_block_number,
        )
        .await?;
        // commit transaction
        txn.commit().await?;
    }
}

/// join activity event filter task
async fn join_activity_event_filter_task(event_type: String, chain_type: String) -> Result<()> {
    let db_pool = get_connection_pool().await?;
    let block_record = get_block_number_by_event_type(
        db_pool.clone(),
        event_type.to_string(),
        chain_type.to_string(),
    )
    .await
    .unwrap();
    // new schedule task
    let mut interval = interval(Duration::from_secs(block_record.sechdule_time as u64));
    loop {
        interval.tick().await;
        let block_record = get_block_number_by_event_type(
            db_pool.clone(),
            event_type.to_string(),
            chain_type.to_string(),
        )
        .await
        .unwrap();
        let provider = Provider::<Ws>::connect(block_record.wss_url).await?;
        let space_block = block_record.space_block;
        let to_block = block_record.last_block + space_block;
        // fetch log
        let logs = fetch_log(
            provider.clone(),
            block_record.contract_addr.clone(),
            block_record.event_signature.clone(),
            block_record.last_block,
            to_block,
        )
        .await?;
        let current_block_number = provider.get_block_number().await?;
        // decode logs
        let decoded_logs: Vec<ethers::abi::Log> =
            parse_event_vec_log(logs.clone(), &block_record.event_signature).unwrap();
        // parse log
        let mut txn = db_pool.begin().await?;
        for (index, _value) in decoded_logs.iter().enumerate() {
            let block_number = logs[index].block_number.unwrap().as_u64() as i32;
            let user_address = decoded_logs[index].params[0].value.clone().to_string();
            // user_address to Address
            let user_address = Address::from(user_address.parse::<Address>()?);
            let user_address = format!("{:?}", user_address);
            let activity_id = decoded_logs[index].params[1].value.clone().to_string();
            let is_winner = decoded_logs[index].params[2].value.clone().to_string();
            // hex 16 to 10
            let activity_id = hex_to_decimal(activity_id.as_str()).unwrap();
            // true or false to 1 or 0
            let is_winner = is_winner == "true".to_string();
            let is_winner = if is_winner { 1 } else { 0 };
            let transaction: H256 = logs[index].transaction_hash.unwrap();
            // get block time
            // let block_time = get_block_time(U64([block_number as u64])).await;
            // // if block_time is None, skip this loop
            // if block_time.is_none() {
            //     continue;
            // }
            // let block_time = block_time.unwrap();
            let block_time = 0;
            // upsert block number record
            println!("user_address = {:?} activity_id = {:?} is_winner = {:?} transaction = {:?} block_number = {:?} block_time = {:?}", user_address.clone(), activity_id as i64, is_winner as i64, format!("{:?}", transaction), block_number, block_time);
        }
        // get min block number
        let min_block_number = min_block_number(current_block_number, to_block);
        // upsert block number record
        let _result = BlockNumberRecord::upsert_last_block_number_chain_event(
            &mut txn,
            event_type.clone(),
            chain_type.clone(),
            min_block_number,
        )
        .await?;
        // commit transaction
        txn.commit().await?;
    }
}

// create space event filter task
async fn create_space_event_filter_task(event_type: String, chain_type: String) -> Result<()> {
    // new db pool
    let db_pool = get_connection_pool().await?;
    let block_record = get_block_number_by_event_type(
        db_pool.clone(),
        event_type.to_string(),
        chain_type.to_string(),
    )
    .await
    .unwrap();
    // new schedule task
    let mut interval = interval(Duration::from_secs(block_record.sechdule_time as u64));
    loop {
        interval.tick().await;
        let block_record = get_block_number_by_event_type(
            db_pool.clone(),
            event_type.to_string(),
            chain_type.to_string(),
        )
        .await
        .unwrap();
        let provider = Provider::<Ws>::connect(block_record.wss_url).await?;
        let client = Arc::new(provider.clone());
        let space_block = block_record.space_block;
        let to_block = block_record.last_block + space_block;
        // fetch log
        let logs = fetch_log(
            provider.clone(),
            block_record.contract_addr.clone(),
            block_record.event_signature.clone(),
            block_record.last_block,
            to_block,
        )
        .await?;
        // parse log
        let mut txn = db_pool.begin().await?;
        for log in logs {
            let block_number_index = log.block_number.unwrap().as_u64() as i32;
            let log_data = log.data;
            // slice log_data on 32 bytes
            let mut log_data_vec: Vec<&[u8]> = Vec::new();
            for i in 0..log_data.len() / 32 {
                let start = i * 32;
                let end = (i + 1) * 32;
                let slice = &log_data[start..end];
                log_data_vec.push(slice);
            }
            // get index 3 from log_data_vec to bytes
            let address_bytes = log_data_vec[3];
            // address_bytes to H256
            let address_bytes = H256::from_slice(address_bytes);
            // H256 to Address
            let owner_address = Address::from(address_bytes);
            let transaction_hash: H256 = log.transaction_hash.unwrap();
            // let block_time = get_block_time(U64([block_number_index as u64]))
            //     .await
            //     .unwrap();
            // upsert user points
        }
        // get current block number
        let current_block_number = client.get_block_number().await?;
        // get min block number
        let min_block_number = min_block_number(current_block_number, to_block);
        // upsert block number record
        let _result = BlockNumberRecord::upsert_last_block_number_chain_event(
            &mut txn,
            event_type.clone(),
            chain_type.clone(),
            min_block_number,
        )
        .await?;
        // commit transaction
        txn.commit().await?;
    }
}

fn min_block_number(current_block_number: U64, to_block_number: i32) -> i32 {
    let current_block_number_i32 = current_block_number.as_u64() as i32;
    if to_block_number > current_block_number_i32 {
        // if to_block_number is too large, use current_block_number
        current_block_number_i32
    } else {
        to_block_number
    }
}

async fn fetch_log(
    provider: Provider<Ws>,
    contract_address: String,
    event_signature: String,
    from_block: i32,
    to_block: i32,
) -> Result<Vec<ethers::types::Log>> {
    let client = Arc::new(provider);
    let filter = Filter::new()
        .address(contract_address.parse::<Address>()?)
        .event(&parse_event_str(event_signature.as_str()))
        .from_block(from_block)
        .to_block(to_block);
    let logs: Vec<ethers::types::Log> = client.get_logs(&filter).await.unwrap();
    Ok(logs)
}

// test fetch log
#[tokio::test]
pub async fn fetch_log_test() {
    let provider = Provider::<Ws>::connect(
        "wss://bsc-mainnet.nodereal.io/ws/v1/3ddb7e8e535747c4a2209cdace70ff29",
    )
    .await
    .unwrap();
    let contract_address = "0x91db77bBe3a79b654137f58157C41267A9830792";
    let event_signature = "event WinnerErc20Amount(address indexed user,uint256 indexed activityId,uint256 indexed amount)";
    let from_block = 33364491;
    let to_block = 33364491 + 40000;
    let logs = fetch_log(
        provider.clone(),
        contract_address.to_string(),
        event_signature.to_string(),
        from_block,
        to_block,
    )
    .await
    .unwrap();
    println!("logs = {:?}", logs);
}

// parse event by event Vec<Log> by event signature
fn parse_event_vec_log(
    logs: Vec<ethers::core::types::Log>,
    event_signature: &str,
) -> Result<Vec<ethers::abi::Log>> {
    let abi: ethers::abi::Abi = ethers::abi::parse_abi(&[event_signature]).unwrap();
    let event_name = event_signature
        .clone()
        .replace("event", "")
        .trim()
        .to_string();
    let start_index = event_name.find("(").unwrap();
    let event_name = &event_name[..start_index].trim();
    let event_instance = abi.events_by_name(event_name).unwrap().first().unwrap();
    // map logs and collect
    let decode_logs: Vec<ethers::abi::Log> = logs
        .iter()
        .map(|log| {
            let raw_log = RawLog::from(log.clone());
            let decoded_log: ethers::abi::Log = event_instance.parse_log_whole(raw_log).unwrap();
            decoded_log
        })
        .collect();
    Ok(decode_logs)
}

fn parse_event_str(event_signature: &str) -> String {
    let mut event = event_signature.to_string();
    event = event.replace("event", "");
    event = event.replace("indexed", "");
    let start_index = match event.find("(") {
        Some(index) => index + 1,     // Skip the opening parenthesis
        None => return String::new(), // Return an empty string if '(' is not found
    };
    let end_index = match event.rfind(")") {
        Some(index) => index,
        None => return String::new(), // Return an empty string if ')' is not found
    };
    let parameters = &event[start_index..end_index];
    let param_pairs: Vec<&str> = parameters.split(',').collect();
    let formatted_parameters: Vec<String> = param_pairs
        .iter()
        .map(|param_pair| {
            let parts: Vec<&str> = param_pair.trim().split(' ').collect();
            if parts.len() > 1 {
                parts[0].to_string()
            } else {
                param_pair.to_string()
            }
        })
        .collect();
    // replace content from start_index to end_index
    event.replace_range(start_index..end_index, &formatted_parameters.join(","));
    // remove space
    event = event.replace(" ", "");
    event
}

async fn get_connection_pool() -> Result<DBPool, sqlx::Error> {
    // load .env
    dotenv().ok();
    // get database url
    let database_url_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = DBPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(2))
        .max_connections(100)
        .connect(&database_url_str)
        .await?;
    Ok(pool)
}

async fn get_block_number_by_event_type(
    db_pool: DBPool,
    event_type: String,
    chain_type: String,
) -> Result<BlockNumberRecord, sqlx::Error> {
    // get latest block number by event_type sql
    let query_res = BlockNumberRecord::get_block_number_by_event_type(
        &db_pool,
        event_type.clone(),
        chain_type.clone(),
    )
    .await?;
    Ok(query_res)
}

async fn get_block_records_by_chain_type(
    db_pool: DBPool,
    chain_type: String,
) -> Result<Vec<BlockNumberRecord>, sqlx::Error> {
    // get block records by chain_type sql
    let query_res =
        BlockNumberRecord::get_block_records_by_chain_type(&db_pool, chain_type).await?;
    Ok(query_res)
}

fn hex_to_decimal(hex_string: &str) -> Result<u64, ParseIntError> {
    let decimal_result = u64::from_str_radix(hex_string, 16);
    decimal_result
}
