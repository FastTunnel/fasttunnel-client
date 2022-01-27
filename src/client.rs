use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use log::info;
use tokio::sync::mpsc::unbounded_channel;
use tokio_tungstenite::{connect_async};
use tokio_tungstenite::tungstenite::handshake::client::Request;
use tokio_tungstenite::tungstenite::protocol;

use crate::config::ClientSettings;

#[derive(Debug)]
pub struct Client {
    config: Arc<ClientSettings>,
}

impl Client {
    #[inline]
    pub fn new(conf: ClientSettings) -> Self {
        let client = Client { config: Arc::new(conf) };
        client
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let (tx, mut rx) = unbounded_channel::<protocol::Message>();
        let request = Request::builder()
            .uri(format!("ws://{}:{}", self.config.server.server_addr.as_str(), self.config.server.server_port))
            .header("FT_VERSION", self.config.version.as_ref().unwrap())
            .header("FT_TOKEN", self.config.token.as_ref().unwrap())
            .body(())?;
        let (ws_stream, _resp) = connect_async(request).await?;
        info!("WebSocket handshake has been successfully completed");

        let (mut write, mut read) = ws_stream.split();

        // login
        tokio::spawn(async move {
            let message = protocol::Message::text("");
            write.send(message).await.unwrap();
        }).await?;

        for x in read.next().await {
            tx.send(x?)?;
        }
        Ok(())
    }

    pub async fn close(&self) -> anyhow::Result<()> {
        Ok(())
    }
}


