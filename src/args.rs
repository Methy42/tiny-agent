use clap::Parser;

/// 一个小型的agent，可以作为socket服务端，也可以作为socket客户端
/// 无论是客户端还是服务端都将收到的消息作为子进程命令行执行
#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// 当前agent作为socket服务端，默认作为服务端
    #[arg(long, default_value_t = true)]
    pub server: bool,

    /// 当前agent作为socket客户端
    #[arg(long)]
    pub client: bool,

    /// 作为服务端是socket监听端口，作为客户端是服务端的连接端口，默认8080
    #[arg(long, default_value_t = 8080)]
    pub server_port: u16,

    /// 服务端IP地址
    #[arg(long)]
    pub server_ip: String,

    /// 是否带socket心跳，默认不带
    #[arg(long)]
    pub heartbeat: bool,

    /// socket心跳间隔，默认10000毫秒
    #[arg(long, default_value = "10000")]
    pub heartbeat_interval: u128,

    /// socket心跳超时时长，默认30000毫秒
    #[arg(long, default_value = "30000")]
    pub heartbeat_timeout: u128,

    /// 作为客户端时socket是否带掉线重试机制，默认不带
    #[arg(long)]
    pub offline_retry: bool,

    /// 作为客户端时socket掉线重试机制间隔，默认30000毫秒
    #[arg(long, default_value = "30000")]
    pub offline_retry_interval: u128,
}
