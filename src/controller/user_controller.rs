// login controller

use axum::{
    routing::{get, post},
    Json, Router,
};
use axum_jwt_auth::Claims;
use hello_axum::state::AppState;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct UserLoginReq {
    pub wallet_address: String,
    pub password: String,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct UserLoginResp {
    pub success: bool,
    pub msg: String,
    pub wallet_address: Option<String>,
    pub login_status: Option<bool>,
    pub jwt_token: Option<String>,
}

// my claims
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MyClaims {
    pub wallet_address: String,
    pub exp: u64,
}

// define router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/userLogin", post(user_login_handler))
        .route("/getUserInfo", get(get_user_info_handler))
}

#[utoipa::path(post, path = "/userLogin", 
    request_body = UserLoginReq,
    responses(
        (status = 200 , description = "login user ", body = [UserLoginResp]),
    )
)]
// user login controller
pub async fn user_login_handler(Json(user_login_param): Json<UserLoginReq>) -> Json<UserLoginResp> {
    // get user login param
    let header = Header::new(Algorithm::HS256);
    // 生成jwt token
    let keys = EncodingKey::from_secret("secret".as_ref());
    let exp_time = Utc::now().timestamp() + 10000000000;
    // encode user info
    let my_claims = MyClaims {
        wallet_address: user_login_param.wallet_address.clone(),
        exp: exp_time as u64,
    };
    let jwt_token = encode::<MyClaims>(&header, &my_claims, &keys).unwrap();
    // return user info
    let user_login_resp = UserLoginResp {
        success: true,
        msg: "login success".to_string(),
        wallet_address: Some(user_login_param.wallet_address.clone()),
        login_status: Some(true),
        jwt_token: Some(jwt_token),
    };
    Json(user_login_resp)
}

// get user info
#[utoipa::path(get, path = "/getUserInfo", 
    responses(
        (status = 200 , description = "get user info ", body = [MyClaims]),
    ),
    security (
        (),
        ("jwt" = [])
    )
)]
pub async fn get_user_info_handler(Claims(my_claims): Claims<MyClaims>) -> Json<MyClaims> {
    // return user info
    Json(my_claims)
}
