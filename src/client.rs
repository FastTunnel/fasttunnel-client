use std::string::String;
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use log::info;
use tokio::sync::{mpsc};
use tokio_tungstenite::{connect_async_with_config};
use tokio_tungstenite::tungstenite::handshake::client::{Request};
use tokio_tungstenite::tungstenite::{Message, protocol};
use serde::{Deserialize, Serialize};
use crate::config::{ClientSettings, Forward, Web};

#[derive(Debug)]
pub struct Client {
    config: Arc<ClientSettings>,
    stop: tokio::sync::watch::Receiver<()>,
    sock: tokio::sync::mpsc::Sender<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginMsg {
    pub webs: Vec<Web>,
    pub forwards: Vec<Forward>,
}

impl Client {
    /// 创建 client
    pub async fn new(conf: ClientSettings, stop_r: tokio::sync::watch::Receiver<()>) -> Self {
        let mut stop_cmd = stop_r.clone();
        let (sock_s, mut sock_r) = mpsc::channel::<String>(100);
        let client = Client {
            config: Arc::new(conf),
            stop: stop_r,
            sock: sock_s,
        };
        tokio::spawn(async move {
            loop {
                tokio::select! {
                   _ =  stop_cmd.changed() => {
                        info!("stop !!!!!!");
                        break;
                    }
                    s = sock_r.recv() => {
                        todo!("处理sock交换")
                    }
                }
            }
        });
        client
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let ip = dns_lookup::lookup_host(self.config.server.server_addr.as_str())
            .map(|ips| ips.into_iter().last())?.expect("dns 寻址失败");
        let url = format!("ws://{}:{}", ip.to_string(), self.config.server.server_port);
        let timeout = self.config.as_ref().server.timeout as u64;
        info!("url:{}",url);
        let request = Request::builder()
            .uri(url)
            .header("FT_VERSION", self.config.version.as_ref().expect("版本配置错误"))
            .header("FT_TOKEN", self.config.token.as_ref().expect("token 配置错误"))
            .body(())?;
        let (ws_stream, _resp) =
            match tokio::time::timeout(tokio::time::Duration::from_secs(timeout), connect_async_with_config(request, None)).await? {
                Ok(data) => { data }
                Err(e) => { return Err(anyhow::Error::from(e)); }
            };
        let (mut write, mut read) = ws_stream.split();
        info!("WebSocket handshake has been successfully completed");

        {
            let login_msg = LoginMsg { webs: self.config.webs.clone(), forwards: self.config.forwards.clone() };
            write.send(Message::from(serde_json::to_string(&login_msg)?)).await?;
        }

        // 获取服务端消息
        for x in read.next().await {
            info!("-----------{}",x?);
        }
        Ok(())
    }

    pub async fn close(&self) -> anyhow::Result<()> {
        todo!("释放资源");
        Ok(())
    }
}


