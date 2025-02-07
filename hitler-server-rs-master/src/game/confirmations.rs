use serde::{Deserialize, Serialize}; // Serde kütüphanesinden Deserialize ve Serialize traitlerini içe aktarır, struct'ı serileştirmek ve serileştirmeyi kaldırmak için kullanılır.
use super::MAX_PLAYERS; // Üst modülden MAX_PLAYERS sabitini içe aktarır.

// Her oyuncunun onay durumunu takip eden struct,
// böylece tüm oyuncular ilerlemeyi seçene kadar oyun devam edemez.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)] // Struct'a clone, copy, serialize, deserialize ve debug özelliklerini kazandırır.
pub struct Confirmations {
    num_players: usize, // Oyun ilerlemesi için gereken oyuncu sayısını tutar.
    state: [bool; MAX_PLAYERS], // Her oyuncunun onay durumunu takip eden bir dizi.
}

impl Confirmations {
    // Yeni bir `Confirmations` struct'ı oluşturur,
    // `num_players` ise ilerlemek için gereken onay sayısını belirler.
    pub fn new(num_players: usize) -> Self {
        let state = [false; MAX_PLAYERS]; // Durum dizisini false ile başlatır, yani hiçbir oyuncu henüz onaylamamış.
        Self { num_players, state } // Yeni bir Confirmations örneği döner.
    }

    // Verilen oyuncunun onay kaydını yapıp yapmadığını döner.
    pub fn has_confirmed(&self, player_idx: usize) -> bool {
        self.state[player_idx] // Verilen oyuncunun onay durumunu döner.
    }

    // Bir oyuncunun onayını kaydeder ve yalnızca oyun şimdi ilerleyebiliyorsa `true` döner.
    pub fn confirm(&mut self, player_idx: usize) -> bool {
        self.state[player_idx] = true; // Verilen oyuncunun onay durumunu true olarak ayarlar.
        self.can_proceed() // Oyunun şimdi ilerleyip ilerleyemeyeceğini kontrol eder ve döner.
    }

    // Oyun şimdi ilerleyebiliyorsa `true` döner.
    pub fn can_proceed(&self) -> bool {
        if std::env::var("QUICK_MODE").is_ok() { // QUICK_MODE ortam değişkeni ayarlanmışsa kontrol eder.
            self.state.iter().any(|c| *c) // QUICK_MODE'da, en az bir oyuncu onayladıysa oyun ilerleyebilir.
        } else {
            self.state.iter().filter(|c| **c).count() >= self.num_players // Aksi takdirde, oyun yalnızca onay sayısı num_players'a eşit veya büyükse ilerler.
        }
    }
}