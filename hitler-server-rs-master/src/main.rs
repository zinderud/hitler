use crate::session::SessionManager;
use crate::ws::accept_connection;
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};
use tokio::net::TcpListener;

mod api;
mod client;
mod error;
mod game;
mod session;
mod ws;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // .env dosyasını yükler
    env_logger::try_init().ok(); // Logger'ı başlatır

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000); // Varsayılan port 3000

    // Socket'e bağlanma
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
    let listener = TcpListener::bind(addr).await.unwrap_or_else(|err| {
        log::error!("Adrese bağlanamadı {:?}: {:?}", addr, err);
        std::process::exit(1)
    });
    log::info!("Dinleniyor: {:?}", addr);

    let db = sled::open("data").unwrap_or_else(|err| {
        log::error!("Veritabanı açılamadı: {:?}", err);
        std::process::exit(1)
    });

    // Oturum yöneticisini oluşturma
    let manager = create_session_manager(db.clone()).unwrap_or_else(|err| {
        log::error!("Oturum yöneticisi oluşturulamadı: {:?}", err);
        std::process::exit(1)
    });
    log::info!("Oturum yöneticisi oluşturuldu. Yüklenen oyun sayısı: {}.", manager.num_games());

    // Eski oyunları temizlemek için arka planda çalışan görev
    tokio::spawn(async {
        loop {
            tokio::task::spawn_blocking(|| manager.purge_games());
            tokio::time::sleep(Duration::from_secs(15)).await;
        }
    });

    // API sunucusu
    if let Some(port) = std::env::var("API_PORT").ok().and_then(|s| s.parse::<u16>().ok()) {
        tokio::spawn(async move {
            let router = api::make_router(manager).await;
            let listener = api::listen(port).await;
            axum::serve(listener, router).await.unwrap_or_else(|err| {
                log::error!("API sunucusu başlatılamadı: {}", err);
            });
        });
    }

    // Bağlantıları kabul etme
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, manager));
    }
}

fn create_session_manager(db: sled::Db) -> Result<&'static SessionManager, Box<dyn Error>> {
    let manager = SessionManager::new(db)?;
    Ok(Box::leak(Box::new(manager)))
}