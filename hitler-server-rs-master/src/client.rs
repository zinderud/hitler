// Gerekli modülleri ve bağımlılıkları kullan
use crate::{
    error::GameError,
    game::{Game as GameInner, GameOptions},
    session::{GameLifecycle, GameUpdate, SessionHandle, SessionManager},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::watch;

// Tek bir oyun istemcisi, bir tahta veya oyuncu olabilir.
pub struct Client<'a> {
    manager: &'a SessionManager, // Oyun oturumlarını yöneten yönetici
    session: Option<SessionHandle>, // Oyun oturumu
    player: Option<String>, // Oyuncu ismi
    game_id: Option<String>, // Oyun kimliği
    updates: Option<watch::Receiver<GameUpdate>>, // Oyun güncellemeleri
}

// Tahta tarafından gerçekleştirilen bir eylem.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BoardAction {
    EndVoting, // Oylamayı sonlandır
    EndCardReveal, // Kart açmayı sonlandır
    EndExecutiveAction, // Yürütme eylemini sonlandır
    EndLegislativeSession, // Yasama oturumunu sonlandır
    EndAssassination, // Suikasti sonlandır
    EndCommunistStart, // Komünist başlangıcı sonlandır
    EndCommunistEnd, // Komünist bitişi sonlandır
    StartSpecialElection, // Özel seçim başlat
}

// Oyuncu tarafından gerçekleştirilen bir eylem.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PlayerAction {
    EndNightRound, // Gece turunu sonlandır
    EndCardReveal, // Kart açmayı sonlandır
    EndExecutiveAction, // Yürütme eylemini sonlandır
    ChoosePlayer { name: String }, // Oyuncu seç
    CastVote { vote: bool }, // Oy kullan
    Discard { index: usize }, // Kart at
    VetoAgenda, // Veto ajandası
    AcceptVeto, // Veto kabul et
    RejectVeto, // Veto reddet
    StartAssassination, // Suikast başlat
    EndCongress, // Kongreyi sonlandır
    HijackElection, // Seçimi ele geçir
}

impl<'a> Client<'a> {
    // Yeni bir oyun istemcisi oluşturur.
    pub fn new(manager: &'a SessionManager) -> Self {
        Self {
            manager,
            session: None, // Başlangıçta oturum yok
            game_id: None, // Başlangıçta oyun kimliği yok
            player: None, // Başlangıçta oyuncu yok
            updates: None, // Başlangıçta güncelleme yok
        }
    }

    // Yeni bir oyun oturumu oluşturur ve kimliğini döndürür.
    pub fn create_game(&mut self, options: GameOptions) -> Result<String, GameError> {
        let session = self.manager.create_game(options)?; // Oyun oturumu oluştur
        let id = session.lock().unwrap().id().to_owned(); // Oyun kimliğini al
        Ok(id) // Oyun kimliğini döndür
    }

    // Bir oyuna tahta olarak katılır.
    pub fn join_as_board(&mut self, game_id: &str) -> Result<(), GameError> {
        let session = self.manager.find_game(game_id)?; // Oyun oturumunu bul
        self.player = None; // Oyuncu yok
        self.game_id = Some(game_id.to_string()); // Oyun kimliğini ayarla
        self.updates = Some(session.lock().unwrap().subscribe()); // Güncellemeleri al
        self.session = Some(session); // Oturumu ayarla
        Ok(())
    }

    // Bir oyuna oyuncu olarak katılır.
    pub fn join_as_player(&mut self, game_id: &str, name: &str) -> Result<(), GameError> {
        let session = self.manager.find_game(game_id)?; // Oyun oturumunu bul
        {
            let mut session = session.lock().unwrap();
            session.add_player(name)?; // Oyuncu ekle
            self.player = Some(name.to_string()); // Oyuncu ismini ayarla
            self.game_id = Some(game_id.to_string()); // Oyun kimliğini ayarla
            self.updates = Some(session.subscribe()); // Güncellemeleri al
        }
        self.session = Some(session); // Oturumu ayarla
        Ok(())
    }

    // Oyun durumunda bir güncelleme olana kadar bekler ve ardından en son durumu döndürür.
    pub async fn next_state(&mut self) -> Value {
        let Some(updates) = &mut self.updates else {
            return std::future::pending().await; // Güncelleme yoksa bekle
        };

        updates.changed().await.ok(); // Güncelleme değişikliklerini bekle
        let update = updates.borrow(); // Güncellemeyi al

        // Oyun durumuna göre durumu belirle
        let state = match update.lifecycle {
            GameLifecycle::Lobby { can_start } => {
                json!({ "type": "lobby", "can_start": can_start }) // Lobide ise durumu döndür
            }
            GameLifecycle::Playing => {
                if let Some(name) = &self.player {
                    let mut state = json!(update.player_updates.iter().find(|u| &u.name == name)); // Oyuncu güncellemesini bul
                    state["type"] = "player".into(); // Oyuncu türü olarak ayarla
                    state
                } else {
                    let mut state = json!(update.board_update); // Tahta güncellemesini al
                    state["type"] = "board".into(); // Tahta türü olarak ayarla
                    state
                }
            }
            GameLifecycle::Ended => json!({ "type": "ended" }), // Oyun bitti ise durumu döndür
        };

        // Oyun kimliği, oyuncu ismi, oyuncular ve durumu döndür
        json!({
            "game_id": self.game_id,
            "name": self.player,
            "players": update.players,
            "state": state
        })
    }

    // Oyundan ayrılır.
    pub fn leave(&mut self) {
        self.player = None; // Oyuncuyu sıfırla
        self.game_id = None; // Oyun kimliğini sıfırla
        self.updates = None; // Güncellemeleri sıfırla
        self.session = None; // Oturumu sıfırla
    }

    // Yeni bir Secret Hitler oyunu başlatır.
    pub fn start_game(&self) -> Result<(), GameError> {
        let Some(session) = &self.session else {
            return Err(GameError::InvalidAction); // Oturum yoksa hata döndür
        };
        let mut session = session.lock().unwrap();
        session.start_game() // Oyunu başlat
    }

    // Tahta bir eylem gerçekleştirdiğinde çağrılır.
    pub fn board_action(&self, action: BoardAction) -> Result<(), GameError> {
        if self.player.is_some() {
            return Err(GameError::InvalidAction); // Oyuncu varsa hata döndür
        }
        self.mutate_game(|game| match action {
            BoardAction::EndVoting => game.end_voting(), // Oylamayı sonlandır
            BoardAction::EndCardReveal => game.end_card_reveal(None), // Kart açmayı sonlandır
            BoardAction::EndExecutiveAction => game.end_executive_action(None), // Yürütme eylemini sonlandır
            BoardAction::EndLegislativeSession => game.end_legislative_session(), // Yasama oturumunu sonlandır
            BoardAction::EndAssassination => game.end_assassination(), // Suikasti sonlandır
            BoardAction::EndCommunistStart => game.end_communist_start(), // Komünist başlangıcı sonlandır
            BoardAction::EndCommunistEnd => game.end_communist_end(), // Komünist bitişi sonlandır
            BoardAction::StartSpecialElection => game.start_special_election(), // Özel seçim başlat
        })
    }

    // Oyuncu bir eylem gerçekleştirdiğinde çağrılır.
    pub fn player_action(&self, action: PlayerAction) -> Result<(), GameError> {
        let player = self.player.as_ref().ok_or(GameError::InvalidAction)?; // Oyuncu yoksa hata döndür
        self.mutate_game(|game| {
            let player = game.find_player(player)?; // Oyuncuyu bul
            match &action {
                PlayerAction::EndNightRound => game.end_night_round(player), // Gece turunu sonlandır
                PlayerAction::EndCardReveal => game.end_card_reveal(Some(player)), // Kart açmayı sonlandır
                PlayerAction::EndExecutiveAction => game.end_executive_action(Some(player)), // Yürütme eylemini sonlandır
                PlayerAction::CastVote { vote } => game.cast_vote(player, *vote), // Oy kullan
                PlayerAction::ChoosePlayer { name } => {
                    let other = game.find_player(name)?; // Diğer oyuncuyu bul
                    game.choose_player(player, other) // Oyuncuyu seç
                }
                PlayerAction::Discard { index } => game.discard_policy(player, *index), // Kart at
                PlayerAction::VetoAgenda => game.veto_agenda(player), // Veto ajandası
                PlayerAction::AcceptVeto => game.veto_agenda(player), // Veto kabul et
                PlayerAction::RejectVeto => game.reject_veto(player), // Veto reddet
                PlayerAction::StartAssassination => game.start_assassination(player), // Suikast başlat
                PlayerAction::EndCongress => game.end_congress(player), // Kongreyi sonlandır
                PlayerAction::HijackElection => game.hijack_special_election(player), // Seçimi ele geçir
            }
        })
    }

    // Oyun oturumunu canlı tutar.
    pub fn heartbeat(&self) {
        let Some(session) = &self.session else {
            return; // Oturum yoksa hiçbir şey yapma
        };
        let mut session = session.lock().unwrap();
        session.heartbeat(); // Oturumu canlı tut
    }

    // Oyunu sonlandırır.
    pub fn end_game(&self) -> Result<(), GameError> {
        let Some(session) = &self.session else {
            return Err(GameError::InvalidAction); // Oturum yoksa hata döndür
        };
        let mut session = session.lock().unwrap();
        session.end_game() // Oyunu sonlandır
    }

    // Oyun üzerinde bir eylem gerçekleştirir.
    fn mutate_game<F>(&self, mutation: F) -> Result<(), GameError>
    where
        F: FnOnce(&mut GameInner) -> Result<(), GameError>,
    {
        let Some(session) = &self.session else {
            return Err(GameError::InvalidAction); // Oturum yoksa hata döndür
        };
        let mut session = session.lock().unwrap();
        session.mutate_game(mutation) // Oyunu değiştir
    }
}