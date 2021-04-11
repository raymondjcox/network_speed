use std::time::{Duration, Instant};


const TARGET_URL: &str = "https://upload.wikimedia.org/wikipedia/commons/2/2d/Snake_River_%285mb%29.jpg";
const KNOWN_SIZE: f64 = 5245329.0;
const NUM_BITS: f64 = KNOWN_SIZE * 8.0;

async fn fetch() -> Result<u128, reqwest::Error> {
    let now = Instant::now();

    reqwest::get(TARGET_URL).await?;
    Ok(now.elapsed().as_millis())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seconds = fetch().await? as f64 / 1000.0;
    let speed_bps = NUM_BITS / seconds;
    let speed_kbps = speed_bps / 1024.0;
    let speed_mbps = speed_kbps / 1024.0;

    println!("{:#?}", seconds);
    println!("speed_bps: {:#?}", speed_bps);
    println!("speed_kbps: {:#?}", speed_kbps);
    println!("speed_mbps: {:#?}", speed_mbps);
    Ok(())
}
