use hyper::Client;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client: Client<hyper::client::HttpConnector> = Client::new();
    //let uri = "http://httpbin.org/ip".parse()?;
//"https://wikipedia.org"
let uri: hyper::Uri = "https://wikipedia.org".parse()?;

    let mut resp: hyper::Response<hyper::Body> = client.get(uri).await?;

    println!("Response: {}", resp.status());
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }
    Ok(())
}