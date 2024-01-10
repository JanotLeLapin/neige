mod rest;

use tokio::sync::mpsc;

#[tokio::main]
pub async fn main() {
    let entry = keyring::Entry::new("neige", &whoami::username()).unwrap();
    let token = match std::env::args().skip(1).next() {
        Some(token) => {
            let _ = entry.set_password(&token);
            token
        },
        None => { entry.get_password().expect("could not get password from keyring") },
    };

    let (mut tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move { rest::rest_task(&token, &mut rx).await });

    let user = rest::wrapper::get_current_user(&mut tx).await;
    let other = rest::wrapper::get_user(&mut tx, "957286680795156510").await;
    println!("{user}");
    println!("{other:?}");
}
