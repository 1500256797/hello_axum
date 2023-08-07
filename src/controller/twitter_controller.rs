use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use bb8::Pool;
use hello_axum::{
    redis_manager::{self, RedisConnectionManager},
    state::AppState,
};
use reverse_engineered_twitter_api::ReAPI;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginTwitterReq {
    twitter_name: String,
    twitter_password: String,
    confirmation_code: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginTwitterResp {
    success: bool,
    msg: String,
    twitter_name: Option<String>,
    login_status: Option<bool>,
}

// define router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/searchTwitter", post(search_content_handler))
        .route("/twitterLogin", post(login_twitter_handler))
}

#[utoipa::path(post, path = "/twitterLogin", 
    request_body = LoginTwitterReq,
    responses(
        (status = 200 , description = "login twitter ", body = [LoginTwitterResp]),
    )
)]
pub async fn login_twitter_handler(
    State(redis_pool): State<Pool<RedisConnectionManager>>,
    Json(req): Json<LoginTwitterReq>,
) -> (StatusCode, Json<LoginTwitterResp>) {
    println!("login twitter handler");
    let mut api = ReAPI::new();
    let name = req.twitter_name.clone();
    let pwd = req.twitter_password.clone();

    // If no verification code is required, set it to empty
    let confirmation_code = "";
    api.login(&name, &pwd, confirmation_code).await;

    // check if account is logged in
    let is_logged_in = api.is_logged_in().await;
    match is_logged_in {
        true => {
            let resp = LoginTwitterResp {
                success: true,
                msg: "login twitter success".to_string(),
                twitter_name: Some(req.twitter_name.clone()),
                login_status: Some(true),
            };
            // set loginTwiterResp to redis hash
            let mut conn = redis_pool.get().await.unwrap();
            // set key
            let key = "loginTwitterResp";
            // set field
            let field_name = format!("loginTwitterResp:{}", req.twitter_name.clone());
            // set value
            let value = serde_json::to_string(&resp).unwrap();
            // set key value
            let _: () = redis::cmd("HMSET")
                .arg(key)
                .arg(field_name)
                .arg(value)
                .query_async(&mut *conn)
                .await
                .unwrap();
            (StatusCode::CREATED, Json(resp))
        }
        false => {
            let resp = LoginTwitterResp {
                success: false,
                msg: "login twitter failed".to_string(),
                twitter_name: Some(req.twitter_name.clone()),
                login_status: Some(false),
            };
            (StatusCode::CREATED, Json(resp))
        }
    }
}

// search twitter
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SearchTwitterReq {
    search_content: String,
    search_limit: u8,
    cursor: String,
}

#[utoipa::path(post, path = "/searchTwitter", 
    request_body = SearchTwitterReq,
    responses(
        (status = 200 , description = "search twitter ", body = [String]),
    )
)]
pub async fn search_content_handler(Json(search_content): Json<SearchTwitterReq>) -> String {
    serde_json::to_string(&search_content).unwrap()
}
