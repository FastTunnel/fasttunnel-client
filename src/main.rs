use clap::Parser;

/// 参数
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 配置文件路径
    #[clap(short, long, default_value = "./app.yaml")]
    config: String,

    /// 版本号
    #[clap(short, long, default_value_t = 1)]
    version: u8,
}

#[tokio::main(worker_threads = 2)]
async fn main() {
    let args = Args::parse();
    dbg!(args);
}
