use axum::{routing::get, routing::post, Router};
use dotenv::dotenv;
use hello_axum::state;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use state::AppState;
use tower::ServiceBuilder;
use std::env;
use std::net::SocketAddr;
use utoipa::OpenApi;
use tower_http::cors::CorsLayer;
use utoipa_swagger_ui::SwaggerUi;
mod controller;
mod database;

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
                
            ),
        ),
        tags(
            (name = "hello_axum", description = "axum 模版工程、集成pg、redis、swagger、sqlx")
        )
    )]
    struct ApiDoc;

    // new appstate
    let state = AppState {
        db_pool: get_connection_pool().await.unwrap(),
    };

    // cors
    let cors = CorsLayer::very_permissive();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(|| async { "Hello, world!" }))
        .route(
            "/getPeopleInfo",
            get(controller::people_controller::get_people_info_handler),
        )
        .route(
            "/createPeopleInfo",
            post(controller::people_controller::create_people_handler),
        )
        .route(
            "/getOrders",
            get(controller::order_controller::get_orders_handler),
        )
        .route(
            "/getOrdersV2",
            get(controller::order_controller::v2_get_orders_handler),
        )
        .route(
            "/createOrder",
            post(controller::order_controller::create_order_handler),
        )
        .route(
            "/updateOrder",
            post(controller::order_controller::update_order_handler),
        )
        .route("/twitterLogin", post(controller::twitter_controller::login_twitter_handler))
        .route("/searchTwitter", post(controller::twitter_controller::search_content_handler))
        // with state
        .with_state(state)
        .layer(
            ServiceBuilder::new()
            .layer(cors)
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_connection_pool() -> Result<PgPool, sqlx::Error> {
    // load .env
    dotenv().ok();
    // get database url
    let database_url_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(2))
        .max_connections(100)
        .connect(&database_url_str)
        .await?;
    Ok(pool)
}
