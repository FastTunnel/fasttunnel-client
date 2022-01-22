use clap::Parser;
use env_logger::Env;
use log::trace;

use crate::conf::{CONFIG, init_config, load_config};

mod conf;

/// 参数
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// 配置文件路径
    #[clap(short, long, default_value = "./app.toml")]
    pub(crate) config: String,

    /// 版本号
    #[clap(short, long, default_value_t = 1)]
    pub(crate) version: u8,
}

#[tokio::main(worker_threads = 2)]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    trace!("客户端初始化");
    {
        let args: Args = Args::parse();
        let config = load_config(&args.config).await?;
        init_config(config).await?;
    }
    trace!("全局加载服务地址{}",&CONFIG.lock().await.as_ref().unwrap().server.server_addr);
    Ok(())
}

