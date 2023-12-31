#![allow(unused)]

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http:127.0.0.1:8080")?;

    hc.do_get("/start?name=Jenn").await?.print().await?;
    hc.do_get("/hello/Mike").await?.print().await?;

    Ok(())
}