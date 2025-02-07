use thiserror::Error; // thiserror paketini kullanarak hata yönetimi sağlar

/// [Game] veya [Session] üzerinde geçersiz bir işlem gerçekleştirme girişiminin sonucudur.
#[derive(Error, Debug)] // GameError enumunu Hata ve Debug özellikleriyle türetir
pub enum GameError {
    #[error("geçersiz oyun seçenekleri kombinasyonu")]
    InvalidGameOptions, // Oyun seçeneklerinin geçersiz kombinasyonu hatası
    #[error("oyun mevcut değil")]
    GameNotFound, // Oyun bulunamadı hatası
    #[error("oyunda çok az oyuncu var")]
    TooFewPlayers, // Oyunda yeterli sayıda oyuncu yok hatası
    #[error("oyunda çok fazla oyuncu var")]
    TooManyPlayers, // Oyunda fazla sayıda oyuncu var hatası
    #[error("verilen isimde oyuncu bulunmuyor")]
    PlayerNotFound, // Verilen isimde oyuncu bulunamıyor hatası
    #[error("devam eden bir oyuna katılamazsınız")]
    CannotJoinStartedGame, // Başlamış bir oyuna katılamazsınız hatası
    #[error("bu oyuncu bu eylem için seçilemez")]
    InvalidPlayerChoice, // Bu oyuncu bu eylem için geçersiz seçim hatası
    #[error("geçersiz oyuncu indeksi")]
    InvalidPlayerIndex, // Geçersiz oyuncu indeksi hatası
    #[error("bu eylem oyun aşamasında gerçekleştirilemez")]
    InvalidAction, // Bu eylem mevcut oyun aşamasında yapılamaz hatası
    #[error("geçersiz bir kart seçildi")]
    InvalidCard, // Geçersiz bir kart seçildi hatası
}