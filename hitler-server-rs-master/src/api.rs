use crate::session::SessionManager; // SessionManager modülünü projeden içe aktarır
use axum::{extract::State, http::StatusCode, routing::get, Json, Router}; // axum kütüphanesinden gerekli modülleri içe aktarır
use serde::Serialize; // serde kütüphanesinden Serialize özelliğini içe aktarır
use serde_json::json; // serde_json kütüphanesinden json fonksiyonunu içe aktarır
use tokio::net::TcpListener; // tokio kütüphanesinden TcpListener modülünü içe aktarır

// Asenkron bir fonksiyon olan make_router, bir SessionManager referansı alır ve bir Router döner
pub async fn make_router(manager: &'static SessionManager) -> Router {
    Router::new() // Yeni bir Router oluşturur
        .route("/sessions", get(get_sessions)) // "/sessions" rotasını get_sessions fonksiyonuna yönlendirir
        .route("/pastgames", get(get_past_games)) // "/pastgames" rotasını get_past_games fonksiyonuna yönlendirir
        .with_state(manager) // Router'a SessionManager durumunu ekler
}

// Asenkron bir fonksiyon olan listen, bir port numarası alır ve bir TcpListener döner
pub async fn listen(port: u16) -> TcpListener {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)) // Belirtilen port numarasına bağlanır ve bir TcpListener oluşturur
        .await // Asenkron olarak bekler
        .expect("could not bind to port"); // Eğer bağlanamazsa bir hata mesajı döner
    log::info!("Listening on {}", listener.local_addr().unwrap()); // Başarıyla bağlandıktan sonra log mesajı yazar
    listener // Oluşturulan TcpListener'ı döner
}

// Asenkron bir fonksiyon olan get_sessions, bir State alır ve bir Json döner veya StatusCode hatası döner
async fn get_sessions(State(manager): State<&SessionManager>) -> Result<Json<impl Serialize>, StatusCode> {
    Ok(Json(json!({ // JSON formatında başarılı bir cevap döner
        "num_sessions": manager.num_games() // manager'dan num_games fonksiyonunu çağırarak oyun sayısını döner
    })))
}

// Asenkron bir fonksiyon olan get_past_games, bir State alır ve bir Json döner veya StatusCode hatası döner
async fn get_past_games(State(manager): State<&SessionManager>) -> Result<Json<impl Serialize>, StatusCode> {
    let games: Vec<_> = manager // manager'dan past_games fonksiyonunu çağırarak geçmiş oyunları alır
        .past_games() // Geçmiş oyunları döner
        .iter() // Oyunları iterasyonla döner
        .map(|(id, stats)| { // Her bir oyun için id ve stats değerlerini map fonksiyonuyla işler
            let mut json = serde_json::to_value(stats).unwrap_or(json!({})); // stats değerini JSON formatına çevirir
            json.as_object_mut().unwrap().insert("id".into(), (*id).into()); // JSON nesnesine id değerini ekler
            json // JSON nesnesini döner
        })
        .collect(); // İşlenmiş oyunları bir vektörde toplar

    Ok(Json(json!({ // JSON formatında başarılı bir cevap döner
        "games": games // Oyunlar listesini döner
    })))
}