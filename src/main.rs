use std::time::SystemTime;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

const TARGET_URL: &str = "https://upload.wikimedia.org/wikipedia/commons/2/2d/Snake_River_%285mb%29.jpg";
const OUT_FILE_NAME: &str = "./data/speed.txt";
const KNOWN_SIZE: f64 = 5245329.0;
const NUM_BITS: f64 = KNOWN_SIZE * 8.0;
const SECONDS_WAIT_BETWEEN_FETCHES: u64 = 10;

async fn fetch(now: &SystemTime) -> Result<u128, reqwest::Error> {
    reqwest::get(TARGET_URL).await?;
    Ok(now.elapsed().unwrap().as_millis())
}


#[tokio::main]
async fn main() {
    let mut now = SystemTime::now();
    loop {
        match now.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_secs() > SECONDS_WAIT_BETWEEN_FETCHES {
                    now = SystemTime::now();
                    let seconds = fetch(&now).await.unwrap() as f64 / 1000.0;
                    let speed_bps = NUM_BITS / seconds;
                    let speed_kbps = speed_bps / 1024.0;
                    let speed_mbps = speed_kbps / 1024.0;
                    println!("speed_mbps: {:#?}", speed_mbps);
                    let mut options = OpenOptions::new();
                    let mut f = options.create(true).append(true).open(OUT_FILE_NAME).await.unwrap();
                    f.write_all(format!("{:.2},{}\n", speed_mbps, now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()).as_bytes()).await.unwrap();
                }
            }
            Err(_) => ()
        }
    }
}
