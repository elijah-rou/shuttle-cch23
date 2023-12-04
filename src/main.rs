use axum::{routing::get, routing::post, Router};
mod day_1;
mod day_4;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/*nums", get(day_1::cube_the_bits))
        .route("/4/strength", post(day_4::strength))
        .route("/4/contest", post(day_4::contest));

    Ok(router.into())
}
