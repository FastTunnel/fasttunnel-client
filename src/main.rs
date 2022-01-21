use std::fs::File;
use std::io::Read;

use clap::Parser;
use tokio::io::AsyncReadExt;
use tracing::{info, Level};

use crate::conf::{CONFIG, init_config, load_config};

mod conf;

/// 参数
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// 配置文件路径
    #[clap(short, long, default_value = "./app.yaml")]
    pub(crate) config: String,

    /// 版本号
    #[clap(short, long, default_value_t = 1)]
    pub(crate) version: u8,
}

#[tokio::main(worker_threads = 2)]
async fn main() -> Result<(), anyhow::Error> {
    tracing::subscriber::with_default(
        tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .finish(),
        || {
            info!("初始化客户端");
        });

    // 加载全局配置
    {
        let args: Args = Args::parse();
        let yaml = load_config(&args.config).await?;
        init_config(yaml).await;
    }
    info!("全局加载服务地址{}",&CONFIG.lock().await.as_ref().unwrap().server.server_addr);
    Ok(())
}

