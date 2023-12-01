use axum::{routing::get, Router, extract::Path};
mod day_1;

async fn day_1(
    Path(nums): Path<String>
) -> String {
    let parsed_nums: Vec<i32> = 
        nums.split("/")
        .map(|c| c.parse::<i32>().unwrap_or(0))
        .collect();
    day_1::cube_the_bits(&parsed_nums)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/*nums", get(day_1));

    Ok(router.into())
}
