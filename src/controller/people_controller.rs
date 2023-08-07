// define People struct

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use hello_axum::state::AppState;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PeopleReq {
    name: String,
    age: u8,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct PeopleResp {
    success: bool,
    msg: String,
    name: Option<String>,
    age: Option<u8>,
}

// define router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/createPeopleInfo", post(create_people_handler))
        .route("/getPeopleInfo", get(get_people_info_handler))
}

#[utoipa::path(post, path = "/createPeopleInfo", 
    request_body = PeopleReq,
    responses(
        (status = 200 , description = "create people info ", body = [PeopleResp]),
    )
)]
pub async fn create_people_handler(Json(pp): Json<PeopleReq>) -> (StatusCode, Json<PeopleResp>) {
    // create people
    let people = PeopleResp {
        success: true,
        msg: "create people success".to_string(),
        name: Some(pp.name),
        age: Some(pp.age),
    };
    // return people
    (StatusCode::CREATED, Json(people))
}

#[utoipa::path(get, path = "/getPeopleInfo", 
    responses(
        (status = 200 , description = "get people info ", body = [PeopleResp]),
    )
)]
// get people handler
pub async fn get_people_info_handler() -> String {
    // new people
    let people = PeopleResp {
        success: true,
        msg: "get people success".to_string(),
        name: Some("John".to_string()),
        age: Some(18),
    };
    // serde to json
    let json = serde_json::to_string(&people).unwrap();
    // return json
    json
}
