use tokio::sync::Mutex;
use lazy_static::lazy_static;
use tokio::io::AsyncReadExt;
use yaml_rust::{Yaml, YamlLoader};

lazy_static! {
    pub static ref CONFIG: Mutex<Option<ClientSettings>> = Mutex::new(None);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Server {
    pub server_addr: String,
    pub server_port: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Forward {
    pub local_ip: String,
    pub local_port: u16,
    pub remote_port: u16,
    pub protocol: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Web {
    pub local_ip: String,
    pub local_port: u16,
    pub sub_domain: String,
    pub www: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ClientSettings {
    pub server: Server,
    pub token: Option<String>,
    pub webs: Vec<Web>,
    pub forwards: Vec<Forward>,
}

/// 加载yaml
pub async fn load_config(path: &String) -> Result<Yaml, anyhow::Error> {
    let mut conf = String::new();
    let mut file = tokio::fs::File::open(path).await?;
    file.read_to_string(&mut conf).await;
    let docs = YamlLoader::load_from_str(&conf)?;
    let yaml = docs.into_iter().next().unwrap();
    Ok(yaml)
}

pub async fn init_config(yaml: Yaml) -> Result<(), anyhow::Error> {
    let server_yaml = &yaml["Server"][0];
    let server_addr = server_yaml["ServerAddr"].as_str().unwrap_or("0.0.0.0");
    let server_port = server_yaml["ServerPort"].as_i64().unwrap_or(0) as u16;
    let server = Server {
        server_addr: String::from(server_addr),
        server_port,
    };
    let token_yaml = &yaml["token"];
    let webs_yaml = &yaml["webs"];
    let forwards_yaml = &yaml["forwards"];

    let client_settings = ClientSettings {
        server,
        token: None,
        webs: vec![],
        forwards: vec![],
    };
    CONFIG.lock().await.replace(client_settings);
    Ok(())
}