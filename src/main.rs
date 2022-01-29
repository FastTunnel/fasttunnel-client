use clap::{Arg, Parser};
use env_logger::Env;
use log::{error, info, trace};

mod config;
mod client;

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

#[cfg(target_os = "linux")]
fn main() {
    unimplemented!();
}

#[cfg(target_os = "windows")]
#[tokio::main(worker_threads = 2)]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // 参数解析
    let matches = clap::App::new("fasttunnel_client")
        .version("0.0.1")
        .arg(
            Arg::new("config")
                .long("config")
                .help("请输入文件路径")
                .short('c')
                .default_value("./app.toml")
                .global(true),
        ).get_matches();
    let config = matches.value_of("config").expect("config 不能为空");
    info!("config path : {}",config);

    let config = config::ClientSettings::load_config(config).await?;
    info!("参数转换成功");
    let (stop_s, stop_r) = tokio::sync::watch::channel(());
    info!("客户端初始化");
    let cli = client::Client::new(config, stop_r).await;
    loop {
        tokio::select! {
            Ok(_) = tokio::signal::ctrl_c() => {
                info!("停止服务");
                stop_s.send(())?;
                break;
            },
            Err(e) =  cli.run() => {
                error!("重新连接WebSocket服务，异常原因：{}",e);
                info!("重启服务");
            }
        }
    }

    Ok(())
}

