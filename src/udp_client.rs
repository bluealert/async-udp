use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Instant,
};

fn main() -> std::io::Result<()> {
    stdudp()?;
    tokio();
    glommio();
    monoio();
    Ok(())
}

fn stdudp() -> std::io::Result<()> {
    let addr = format!("127.0.0.1:0");
    let socket = std::net::UdpSocket::bind(addr)?;
    let target = format!("127.0.0.1:10000");
    let start = Instant::now();
    socket.send_to("example.com".as_bytes(), &target)?;
    println!("std send: {}us", start.elapsed().as_micros());
    let mut buf = [0; 1024];
    let start = Instant::now();
    let (_len, _addr) = socket.recv_from(&mut buf)?;
    println!("std recv: {}us\n", start.elapsed().as_micros());
    Ok(())
}

fn tokio() {
    let exe = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .build()
        .unwrap();
    exe.block_on(async move {
        if let Err(err) = run_tokio().await {
            log::error!("{:?}", err);
        }
    });
}

async fn run_tokio() -> std::io::Result<()> {
    let addr = format!("127.0.0.1:0");
    let socket = tokio::net::UdpSocket::bind(addr).await?;
    let target = format!("127.0.0.1:10000");
    let start = Instant::now();
    socket.send_to("example.com".as_bytes(), &target).await?;
    println!("tokio send: {}us", start.elapsed().as_micros());
    let mut buf = [0; 1024];
    let start = Instant::now();
    let (_len, _addr) = socket.recv_from(&mut buf).await?;
    println!("tokio recv: {}us\n", start.elapsed().as_micros());
    Ok(())
}

fn glommio() {
    let exe = glommio::LocalExecutorBuilder::new(glommio::Placement::Unbound)
        .make()
        .unwrap();
    exe.run(async move {
        if let Err(err) = run_glommio().await {
            log::error!("{:?}", err);
        }
    });
}

async fn run_glommio() -> std::io::Result<()> {
    let addr = format!("127.0.0.1:0");
    let socket = glommio::net::UdpSocket::bind(addr)?;
    log::info!("glommio udp {:?}", socket);
    let target = format!("127.0.0.1:10000");
    let start = Instant::now();
    socket.send_to("example.com".as_bytes(), &target).await?;
    println!("glommio send: {}us", start.elapsed().as_micros());
    let mut buf = [0; 1024];
    let start = Instant::now();
    let (_len, _addr) = socket.recv_from(&mut buf).await?;
    println!("glommio recv: {}us\n", start.elapsed().as_micros());
    Ok(())
}

#[monoio::main]
async fn monoio() {
    if let Err(err) = run_monoio().await {
        log::error!("{:?}", err);
    }
}

async fn run_monoio() -> std::io::Result<()> {
    use std::io::Write;

    let addr = format!("127.0.0.1:0");
    let socket = monoio::net::udp::UdpSocket::bind(addr)?;
    log::info!("monoio udp {:?}", socket);
    let target = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10000);
    let mut buf: Vec<u8> = Vec::with_capacity(1024);
    write!(&mut buf, "example.com")?;
    let res;
    let start = Instant::now();
    (res, buf) = socket.send_to(buf, target).await;
    res?;
    println!("monoio send: {}us", start.elapsed().as_micros());
    let start = Instant::now();
    let (res, _buf) = socket.recv_from(buf).await;
    println!("monoio recv: {}us\n", start.elapsed().as_micros());
    if res?.0 == 0 {
        return Ok(());
    }
    Ok(())
}
