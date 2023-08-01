use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Signature {
    pub id: i64,
    pub text_signature: String,
    pub bytes_signature: String,
    pub create_at: String,
    pub update_at: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct SignatureReq {
    pub page_num: u32,
    pub page_size: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SignatureResp {
    pub success: bool,
    pub msg: String,
    pub total: u64,
    pub data: Vec<Option<Signature>>,
}

// 分页查询列表
#[utoipa::path(get, path = "/getSignatures", 
    params (
        SignatureReq,
    ),
    responses(
        (status = 200 , description = "get signatures info ", body = [SignatureResp]),
    )
)]
pub async fn get_signatures_handler(
    Query(signature_param): Query<SignatureReq>,
) -> Json<SignatureResp> {
    todo!()
}

// SignatureWithParamNames

#[utoipa::path(get, path = "/getSignaturesWithParamNames", 
    params (
        SignatureReq,
    ),
    responses(
        (status = 200 , description = "get signatures info ", body = [SignatureResp]),
    )
)]
pub async fn get_signatures_with_param_names_handler(
    Query(signature_param): Query<SignatureReq>,
) -> Json<SignatureResp> {
    todo!()
}

// get signature by bytes_signature
#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct SignatureWithBytesSignatureReq {
    pub bytes_signature: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SignatureWithBytesSignatureResp {
    pub success: bool,
    pub msg: String,
    pub data: Vec<Option<Signature>>,
}
#[utoipa::path(get, path = "/getSignatureByBytesSignature", 
    params (
        SignatureWithBytesSignatureReq,
    ),
    responses(
        (status = 200 , description = "get signature info ", body = [SignatureWithBytesSignatureResp]),
    )
)]
pub async fn get_signature_by_bytes_signature_handler(
    Query(bytes_signature): Query<Signature>,
) -> Json<Signature> {
    todo!()
}
