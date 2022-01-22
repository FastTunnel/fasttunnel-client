use lazy_static::lazy_static;
use serde::Deserialize;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref CONFIG: Mutex<Option<ClientSettings>> = Mutex::new(None);
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Server {
    pub server_addr: String,
    pub server_port: u16,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Forward {
    pub local_ip: String,
    pub local_port: u16,
    pub remote_port: u16,
    pub protocol: Option<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Web {
    pub local_ip: String,
    pub local_port: u16,
    pub sub_domain: String,
    pub www: Vec<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ClientSettings {
    pub server: Server,
    pub token: Option<String>,
    pub webs: Vec<Web>,
    pub forwards: Vec<Forward>,
}

/// 加载yaml
pub async fn load_config(path: &String) -> Result<ClientSettings, anyhow::Error> {
    let mut conf = String::new();
    let mut file = tokio::fs::File::open(path).await?;
    file.read_to_string(&mut conf).await?;
    Ok(toml::from_str(&conf)?)
}

pub async fn init_config(config: ClientSettings) -> Result<(), anyhow::Error> {
    CONFIG.lock().await.replace(config);
    Ok(())
}