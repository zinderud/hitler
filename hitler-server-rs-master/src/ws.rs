use crate::{
    client::{BoardAction, Client, PlayerAction}, // Gerekli modülleri içe aktarır
    error::GameError, // Hata modülünü içe aktarır
    game::GameOptions, // Oyun seçenekleri modülünü içe aktarır
    session::SessionManager, // Oturum yöneticisi modülünü içe aktarır
};
use futures_util::{select, FutureExt, SinkExt, StreamExt, TryStreamExt}; // Futures ve stream işlemleri için gerekli modülleri içe aktarır
use serde::{Deserialize, Serialize}; // Serileştirme ve serileştirmeden çıkarmak için modülleri içe aktarır
use serde_json::json; // JSON işlemleri için gerekli modülü içe aktarır
use tokio::net::TcpStream; // Tokio'nun TCP bağlantı modülünü içe aktarır
use tokio_tungstenite::tungstenite::Message; // WebSocket mesajları için gerekli modülü içe aktarır

// Yeni bir bağlantıyı kabul eder
pub async fn accept_connection(stream: TcpStream, manager: &SessionManager) {
    log::info!("Accepted new connection"); // Yeni bağlantı kabul edildiğini loglar

    let Ok(stream) = tokio_tungstenite::accept_async(stream).await else {
        log::error!("Error occured during websocket handshake"); // WebSocket el sıkışması sırasında hata oluştuğunu loglar
        return;
    };
    let (mut write, read) = stream.split(); // Yazma ve okuma stream'lerini ayırır
    let mut read = read.fuse(); // Okuma stream'ini birleştirir

    let mut client = Client::new(manager); // Yeni bir istemci oluşturur

    loop {
        select! { // Seçim yapar
            msg = read.try_next() => {
                let Ok(Some(Message::Text(msg))) = msg else {
                    break;
                };
                let Ok(msg) = serde_json::from_str::<WsRequest>(&msg) else {
                    log::error!("Cannot parse message: {}", &msg); // Mesajı ayrıştıramadığını loglar
                    break;
                };
                match process_request(msg, &mut client) { // İstemci isteğini işler
                    Ok(()) => {},
                    Err(err) => {
                        let reply = json!({
                            "type": "error",
                            "error": err.to_string()
                        });
                        write.send(Message::Text(reply.to_string())).await.ok(); // Hata mesajını gönderir
                    }
                }
            },
            state = client.next_state().fuse() => { // İstemcinin bir sonraki durumunu alır
                let msg = json!({
                    "type": "update",
                    "state": state
                });
                if write.send(Message::Text(msg.to_string())).await.is_err() {
                    log::error!("Could not send websockets message"); // WebSocket mesajı gönderilemediğini loglar
                    break;
                }
            }
        }
    }
}

// Oyun istemcisi tarafından sunucuya gönderilen bir mesaj.
#[derive(Serialize, Deserialize)]
enum WsRequest {
    CreateGame { options: GameOptions }, // Oyun oluşturma isteği
    JoinAsBoard { game_id: String }, // Tahta olarak katılma isteği
    JoinAsPlayer { game_id: String, name: String }, // Oyuncu olarak katılma isteği
    LeaveGame, // Oyundan ayrılma isteği
    StartGame, // Oyunu başlatma isteği
    BoardAction(BoardAction), // Tahta aksiyonu isteği
    PlayerAction(PlayerAction), // Oyuncu aksiyonu isteği
    Heartbeat, // Kalp atışı isteği
    EndGame, // Oyunu bitirme isteği
}

// İstemciden gelen bir isteği işler.
fn process_request(req: WsRequest, client: &mut Client) -> Result<(), GameError> {
    match req {
        WsRequest::CreateGame { options } => {
            let game_id = client.create_game(options)?; // Oyun oluşturur
            client.join_as_board(&game_id)?; // Tahta olarak katılır
        }
        WsRequest::JoinAsBoard { game_id } => {
            client.join_as_board(&game_id)?; // Tahta olarak katılır
        }
        WsRequest::JoinAsPlayer { game_id, name } => {
            client.join_as_player(&game_id, &name)?; // Oyuncu olarak katılır
        }
        WsRequest::LeaveGame => client.leave(), // Oyundan ayrılır
        WsRequest::StartGame => client.start_game()?, // Oyunu başlatır
        WsRequest::BoardAction(action) => {
            // Hataları açıkça yok sayar çünkü birden fazla oyun tahtası olduğunda hatalar oluşacaktır.
            client.board_action(action).ok(); 
        }
        WsRequest::PlayerAction(action) => client.player_action(action)?, // Oyuncu aksiyonunu gerçekleştirir
        WsRequest::EndGame => client.end_game()?, // Oyunu bitirir
        WsRequest::Heartbeat => client.heartbeat(), // Kalp atışını gönderir
    }
    Ok(())
}