use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let addr = format!("127.0.0.1:10000");
    if let Err(err) = do_echo(&addr).await {
        log::error!("failed: {:?}", err);
    }
    Ok(())
}

async fn do_echo(addr: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind(addr).await?;
    log::info!("Server is listening on {}", addr);
    loop {
        let mut buf = [0; 1024];
        let (len, addr) = socket.recv_from(&mut buf).await?;
        socket.send_to(&buf[..len], addr).await?;
    }
}
