use tokio::sync::{mpsc, oneshot};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use http_body_util::{Empty, BodyExt};

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

pub async fn rest_task(token: &str, rx: &mut mpsc::Receiver<RestRequest<'_>>) {
    while let Some(data) = rx.recv().await {
        let https = hyper_tls::HttpsConnector::new();
        let client = Client::builder(TokioExecutor::new())
            .build::<_, Empty<bytes::Bytes>>(https);
        let req = hyper::Request::builder()
            .uri(format!("https://discord.com/api/v10{}", data.url))
            .header(hyper::header::AUTHORIZATION, token)
            .body(Empty::<bytes::Bytes>::new()).unwrap();
        let mut res = client.request(req).await.unwrap();
        let mut buf = Vec::new();
        while let Some(next) = res.frame().await {
            let frame = next.unwrap();
            if let Some(chunk) = frame.data_ref() {
                buf.append(&mut chunk.to_vec());
            }
        }
        data.res.send(String::from_utf8(buf).unwrap()).unwrap();
    }
}
