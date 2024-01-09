use tokio::sync::{mpsc, oneshot};

pub struct RestRequest<'a> {
    res: oneshot::Sender<String>,
    url: &'a str,
    body: Option<&'a str>,
}

pub async fn send<'a>(tx: &mut mpsc::Sender<RestRequest<'a>>, url: &'a str, body: Option<&'a str>) -> Result<String, tokio::sync::oneshot::error::RecvError> {
    let (res_tx, res_rx) = oneshot::channel();
    tx.send(RestRequest { res: res_tx, url, body }).await.unwrap();
    res_rx.await
}

pub async fn rest_task(rx: &mut mpsc::Receiver<RestRequest<'_>>) {
    while let Some(data) = rx.recv().await {
        println!("Got request for '{}'", data.url);
        data.res.send("foobar".to_string()).unwrap();
    }
}
