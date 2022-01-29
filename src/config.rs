use serde::Deserialize;
use tokio::io::AsyncReadExt;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Server {
    pub server_addr: String,
    pub server_port: u16,
    pub timeout: u32,
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
    pub version: Option<String>,
    pub webs: Vec<Web>,
    pub forwards: Vec<Forward>,
}

impl ClientSettings {
    /// 加载yaml
    pub async fn load_config(path: &str) -> Result<Self, anyhow::Error> {
        let mut conf = String::new();
        let mut file = tokio::fs::File::open(path).await?;
        file.read_to_string(&mut conf).await?;
        Ok(toml::from_str(&conf)?)
    }
}

 