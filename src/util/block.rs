// use cached::proc_macro::cached;
// use chrono::{DateTime, Utc};
// use ethers::providers::{Middleware, ProviderError};
// use ethers::types::U64;
// use ethers::types::{Block, H256};

// /// should only cache the result when the result is `Ok`
// #[cached(result = true)]
// pub async fn get_block(blockno: U64) -> Result<Block<H256>, ProviderError> {
//     match PROVIDER.to_owned().get_block(blockno).await {
//         Err(e) => {
//             println!("get_block error: {:?}", e);
//             Err(e)
//         }
//         Ok(v) => {
//             // if v is none
//             if v.is_none() {
//                 println!("get_block error: block is none");
//                 let err = ProviderError::CustomError("block is none".to_string());
//                 Err(err)
//             } else {
//                 Ok(v.unwrap())
//             }
//         }
//     }
// }

// pub async fn get_block_time(blockno: U64) -> Option<DateTime<Utc>> {
//     match get_block(blockno).await {
//         Err(_) => None,
//         Ok(data) => {
//             let time = data.time();
//             match time {
//                 Ok(time) => Some(time),
//                 Err(e) => {
//                     println!("get_block_time error: {:?}", e);
//                     None
//                 }
//             }
//         }
//     }
// }

// pub async fn get_newest_blockno() -> Result<U64, ProviderError> {
//     PROVIDER.to_owned().get_block_number().await
// }

// #[cfg(test)]
// mod test_block {

//     use crate::util::block::get_block_time;
//     use ethers::types::U64;
//     #[tokio::test]
//     async fn test_block() {
//         dotenv::dotenv().ok();
//         let block = get_block_time(U64([31983237u64])).await;
//         assert!(block.is_some());
//         assert!(block.unwrap().timestamp() as u64 == 1695441666u64);
//     }
// }
