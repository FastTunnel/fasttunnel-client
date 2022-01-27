mod config;
mod client;

use clap::Parser;
use env_logger::Env;
use log::trace;


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
    let args: Args = Args::parse();
    trace!("参数转换成功");

    let config = config::ClientSettings::load_config(&args.config).await?;

    trace!("客户端初始化");
    let cli = client::Client::new(config);
    loop {
        tokio::select! {
            Ok(_) = tokio::signal::ctrl_c() => {
                trace!("stop");
                break;
            },
            _ =  cli.run() => {
                trace!("restart");
            }
        }
    }

    Ok(())
}

