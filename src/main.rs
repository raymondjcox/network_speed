use std::time::{Duration, Instant};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const TARGET_URL: &str = "https://upload.wikimedia.org/wikipedia/commons/2/2d/Snake_River_%285mb%29.jpg";
const OUT_FILE_NAME: &str = "speed";
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
    println!("speed_mbps: {:#?}", speed_mbps);
    let mut options = OpenOptions::new();
    let mut f: tokio::fs::File = options.create(true).append(true).open(OUT_FILE_NAME).await?;
    f.write_all(format!("{:.2}\n", speed_mbps).as_bytes()).await?;
    Ok(())
}
