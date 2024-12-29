use std::time::Duration;

use tokio::{fs::write, time::sleep};

async fn add_one(x: u32) -> u32 {
    println!("Test");
    let y = vec![32, 64, 238];
    write("src/test.txt", x.to_string()).await;
    let z = &y[0];
    sleep(Duration::from_secs(1)).await;
    let a = x + z + 1;
    println!("{}", a);
    return a;
}

#[tokio::main]
async fn main() {
    let f1 = tokio::spawn(add_one(69));
    let f1 = f1.await.unwrap();
    println!("{f1}");

    println!("Hello, world!");
}
