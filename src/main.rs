use std::time::{Duration, Instant};


const TARGET_URL: &str = "https://upload.wikimedia.org/wikipedia/commons/b/b9/Pizigani_1367_Chart_1MB.jpg";
const KNOWN_SIZE: usize = 1093957;
const NUM_BITS: usize = KNOWN_SIZE * 8;

async fn fetch() -> Result<u128, reqwest::Error> {
    let now = Instant::now();

    reqwest::get(TARGET_URL)
        .await?
        .text()
        .await?;

    Ok(now.elapsed().as_millis())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = fetch().await?;
    println!("{:#?}", time);
    Ok(())
}
