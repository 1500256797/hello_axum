use std::{convert::Infallible, time::Duration};

use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router, TypedHeader,
};
use futures::stream::{self, Stream};

use hello_axum::state::AppState;
use tokio_stream::StreamExt as _;

// router
pub fn router() -> Router<AppState> {
    Router::new().route("/sse", get(sse_handler))
}

// sse handler
#[utoipa::path(post, path = "/sse", 
    responses(
        (status = 200 , description = "sse handler ", body = [SseResp]),
    )
)]

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());
    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let stream = stream::repeat_with(|| Event::default().data("hello world".to_owned()))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
    // curl http://xxxx:3000/sse
}
