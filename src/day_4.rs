use serde::{Deserialize, Serialize};
use axum::extract::Json;

#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: u32
}

#[derive(Deserialize)]
struct MegaReindeer {
    name: String,
    strength: u32,
    speed: f64,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favourite_food: String,
    cAnD13s_3ATeN_yesT3rdAy: String
}

#[derive(Serialize)]
struct ContestOutcome {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String
}

pub async fn strength(Json(reindeer): Json<Vec<Reindeer>>) -> String {
    reindeer.iter()
        .map(|r| r.strength)
        .sum::<u32>()
        .to_string()
}

pub async fn contest(Json(reindeer): Json<Vec<MegaReindeer>>) -> axum::Json<ContestOutcome>{
    let r_iter = reindeer.iter();

    let tallest = r_iter.position(
        |&x| x == r_iter
        .map(|r| r.height)
        .max()
        .unwrap()
    );
    let wizard: u32 = 0;
    let capitalist: u32 = 0;
    
    Json(ContestOutcome{
        fastest: format!("Speeding past the finish line with a strength of {} is {}", ),
        tallest: format!("{} is standing tall with his 36 cm wide antlers"),
        magician: format!("{} could blast you away with a snow magic power of 9001"),
        consumer: format!("{} ate lots of candies, but also some {}")
    })
}