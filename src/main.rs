use axum::{routing::get, routing::post, Router};
use bb8::Pool;
use dotenv::dotenv;
use hello_axum::redis_manager::RedisConnectionManager;
use hello_axum::state;
use jsonwebtoken::{ Algorithm,DecodingKey,Validation};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use state::AppState;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::services;
use tracing::Level;
use std::{env, path::PathBuf};
use std::net::SocketAddr;
use utoipa::{OpenApi, Modify, openapi::security::{SecurityScheme, ApiKey, ApiKeyValue}};
use axum_jwt_auth::{JwtDecoderState,LocalDecoder, Decoder};
use utoipa_swagger_ui::SwaggerUi;
mod controller;
mod model;
use tower_http::trace::{self, TraceLayer};

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            controller::people_controller::get_people_info_handler,
            controller::people_controller::create_people_handler,
            controller::order_controller::get_orders_handler,
            controller::order_controller::v2_get_orders_handler,
            controller::order_controller::create_order_handler,
            controller::order_controller::update_order_handler,
            controller::twitter_controller::login_twitter_handler,
            controller::twitter_controller::search_content_handler,
            controller::user_controller::user_login_handler,
            controller::user_controller::get_user_info_handler,
            controller::bytes4_controller::get_signatures_handler,
            controller::bytes4_controller::get_signatures_with_param_names_handler,
            controller::bytes4_controller::get_signature_by_bytes_signature_handler,
        ),
        components(
            schemas(
                controller::people_controller::PeopleReq,
                controller::people_controller::PeopleResp,
                controller::order_controller::CreateOrderReq,
                controller::order_controller::CreateOrderResp,  
                controller::twitter_controller::LoginTwitterReq,
                controller::twitter_controller::LoginTwitterResp,
                controller::twitter_controller::SearchTwitterReq,
                controller::user_controller::UserLoginReq,
                controller::user_controller::UserLoginResp,
                controller::user_controller::MyClaims,
                controller::bytes4_controller::SignatureReq,
                controller::bytes4_controller::SignatureResp,
                controller::bytes4_controller::Signature,
                controller::bytes4_controller::SignatureWithBytesSignatureReq,
                controller::bytes4_controller::SignatureWithBytesSignatureResp,
            ),
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "hello_axum", description = "axum 模版工程、集成pg、redis、swagger、sqlx")
        )
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "jwt",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
                )
            }
        }
    }
    // tracing 
    tracing_subscriber::fmt()
    .with_target(false)
    .pretty()
    .init();

    // load .env
    dotenv().ok();
    // get database url
    let redis_url = env::var("REDIS_URL").expect("😀REDIS_URL must be set");
    let redis_connection_manager = RedisConnectionManager::new(redis_url).unwrap();
    
    let redis_pool = Pool::builder()
    .build(redis_connection_manager).await.unwrap();

    // decoder 
    let keys : Vec<DecodingKey> = vec![DecodingKey::from_secret("secret".as_ref())];
    let validation:Validation= jsonwebtoken::Validation::new(Algorithm::HS256);
    let jwt_decoder : Decoder= LocalDecoder::new(keys, validation).into();
    // new appstate
    let pool = get_connection_pool().await.unwrap();
    // sql will be migrated to db only when axum app start and  sql file changed
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await.unwrap();
    //     // sql migrate
    let state: AppState = AppState {
        db_pool: pool,
        redis_pool: redis_pool,
        jwt_decoder: JwtDecoderState{
            decoder : jwt_decoder
        }
    };
    


    // cors
    let cors = CorsLayer::very_permissive();

    // assets
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let static_files_service = services::ServeDir::new(assets_dir).append_index_html_on_directories(true);
    
    let app = Router::new()
        .fallback_service(static_files_service)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { "请打开swagger文档链接😃:http://localhost/swagger-ui/" }))
        .merge(controller::people_controller::router())
        .merge(controller::order_controller::router())
        .merge(controller::user_controller::router())
        .merge(controller::bytes4_controller::router())
        .merge(controller::twitter_controller::router())
        .merge(controller::sse_controller::router())
        // with state
        .with_state(state)
        .layer(
            ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO)),
                // tracelayer end
            )
            .layer(cors)
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


}

async fn get_connection_pool() -> Result<PgPool, sqlx::Error> {
    // load .env
    dotenv().ok();
    // get database url
    let database_url_str = env::var("DATABASE_URL").expect("😀DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(2))
        .max_connections(100)
        .connect(&database_url_str)
        .await?;
    Ok(pool)
}
