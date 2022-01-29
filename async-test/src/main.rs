async fn learn_song() -> f64 {
    let mut sum: f64 = 0.0;
    for _i in 0..10 {
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        sum += 1 as f64;
    }
    sum
}

async fn sing_song() {
    println!("sing song");
}

async fn dance() {
    tokio::time::sleep(tokio::time::Duration::from_nanos(2)).await;
    println!("dance");
}

async fn learn_and_sing_song() {
    let learned = learn_song();
    println!("i already learned song: {}", learned.await);
    sing_song().await;
}

async fn create_dir() {
    let beg = std::time::Instant::now();
    tokio::fs::create_dir("./test").await;
    println!("create folder consume: {:?}", beg.elapsed().as_micros());
}

async fn async_main() {
    let (_a, _b) = tokio::join!(dance(), create_dir());
}

#[tokio::main]
async fn main() {
    let now = tokio::time::Instant::now();
    async_main().await;
    println!("{:?}", now.elapsed().as_micros());
}
