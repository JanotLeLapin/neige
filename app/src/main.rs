mod rest;

use tokio::sync::mpsc;

#[tokio::main]
pub async fn main() {
    let (mut tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move { rest::rest_task(&mut rx).await });

    let a = rest::send(&mut tx, "/users/@me", None).await.unwrap();
    let b = rest::send(&mut tx, "/users/foo", None).await.unwrap();

    println!("{a}, {b}");
}
