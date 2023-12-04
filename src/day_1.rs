
use axum::extract::Path;

pub async fn cube_the_bits(
    Path(nums): Path<String>
) -> String {
    nums.split("/")
        .map(|c| c.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>()
        .iter()
        .fold(0, |acc, &bit| acc ^ bit)
        .pow(3)
        .to_string()
}