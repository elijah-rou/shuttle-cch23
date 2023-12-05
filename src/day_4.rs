use axum::extract::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
}

#[derive(Deserialize)]
pub struct MegaReindeer {
    name: String,
    strength: u32,
    speed: f64,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies: u32,
}

#[derive(Serialize)]
pub struct ContestOutcome {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub async fn strength(Json(reindeer): Json<Vec<Reindeer>>) -> String {
    reindeer.iter().map(|r| r.strength).sum::<u32>().to_string()
}

pub async fn contest(Json(reindeer): Json<Vec<MegaReindeer>>) -> axum::Json<ContestOutcome> {
    let fastest = reindeer
        .iter()
        .max_by(|r1, r2| r1.speed.total_cmp(&r2.speed))
        .unwrap();
    let tallest = reindeer
        .iter()
        .max_by(|r1, r2| r1.height.cmp(&r2.height))
        .unwrap();
    let wizard = reindeer
        .iter()
        .max_by(|r1, r2| r1.snow_magic_power.cmp(&r2.snow_magic_power))
        .unwrap();
    let capitalist = reindeer
        .iter()
        .max_by(|r1, r2| r1.candies.cmp(&r2.candies))
        .unwrap();

    Json(ContestOutcome {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            wizard.name, wizard.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            capitalist.name, capitalist.favorite_food
        ),
    })
}
