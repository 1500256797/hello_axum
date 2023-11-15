use crate::model;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use hello_axum::state::AppState;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

// define router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/getOrders", get(get_orders_handler))
        .route("/getOrdersV2", get(v2_get_orders_handler))
        .route("/createOrder", post(create_order_handler))
        .route("/updateOrder", post(update_order_handler))
}

#[utoipa::path(get, path = "/getOrders", 
    responses(
        (status = 200 , description = "get orders info ", body = [String]),
    )
)]
pub async fn get_orders_handler(State(db_pool): State<PgPool>) -> String {
    // 查询order
    let quer_as_res = model::order::Orders::get_orders_count(&db_pool)
        .await
        .unwrap();
    // 返回json
    let json: String = serde_json::to_string(&quer_as_res).unwrap();
    json
}

#[utoipa::path(get, path = "/getOrdersV2", 
    responses(
        (status = 200 , description = "get orders info ", body = [String]),
    )
)]
pub async fn v2_get_orders_handler(State(db_pool): State<PgPool>) -> String {
    // 查询order
    let query_res = model::order::Orders::get_orders_count_v2(&db_pool)
        .await
        .unwrap();
    // 返回json
    let json: String = serde_json::to_string(&query_res).unwrap();
    json
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderReq {
    hash: String,
    offerer: String,
    zone: String,
    zone_hash: String,
    start_time: i64,
    end_time: i64,
    order_type: i32,
    total_original_consideration_items: i32,
    salt: String,
    counter: i64,
    conduit_key: String,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderResp {
    success: bool,
    msg: String,
    hash: String,
}

#[utoipa::path(post, path = "/createOrder", 
    request_body = CreateOrderReq,
    responses(
        (status = 200 , description = "create order info ", body = [CreateOrderResp]),
    )
)]
pub async fn create_order_handler(
    State(db_pool): State<PgPool>,
    Json(order_req): Json<CreateOrderReq>,
) -> () {
    // 创建参数
    let order_input = model::order::OrderInput {
        hash: 1234568,
        offerer: "sadsad".to_string(),
        zone: 123123,
        zone_hash: 12321334,
        start_time: 100000000,
        end_time: 20000000000,
        order_type: 1231,
        total_original_consideration_items: 2343,
        salt: "adfadfadf".to_string(),
        counter: 123,
        conduit_key: 123123,
        signature: "adfadfadf".to_string(),
        cancelled: false,
        finalized: false,
        marked_invalid: false,
    };

    // insert
    let quer_res = model::order::OrderInput::insert_order(&db_pool, &order_input).await;

    match quer_res {
        Ok(resut) => {
            println!("insert success: {:?}", resut.rows_affected());
        }
        Err(e) => {
            println!("insert error: {:?}", e);
        }
    }
}

#[utoipa::path(post, path = "/updateOrder", 
    request_body = CreateOrderReq,
    responses(
        (status = 200 , description = "update order info ", body = [String]),
    )
)]
pub async fn update_order_handler(
    State(db_pool): State<PgPool>,
    Json(order_req): Json<CreateOrderReq>,
) -> String {
    // 创建参数
    let order_input = model::order::OrderInput {
        hash: 1234568,
        offerer: "sadsad".to_string(),
        zone: 123123,
        zone_hash: 12321334,
        start_time: 666666,
        end_time: 7777777,
        order_type: 1231,
        total_original_consideration_items: 2343,
        salt: "adfadfadf".to_string(),
        counter: 123,
        conduit_key: 123123,
        signature: "adfadfadf".to_string(),
        cancelled: false,
        finalized: false,
        marked_invalid: false,
    };
    // update
    let quer_res = model::order::OrderInput::update_orders(&db_pool, &order_input).await;
    // match
    match quer_res {
        Ok(resut) => {
            println!("update success: {:?}", resut);
            let json: String = serde_json::to_string(&resut).unwrap();
            return json;
        }
        Err(e) => {
            println!("update error: {:?}", e);
            return "update error".to_string();
        }
    }
}
