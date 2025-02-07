// 'executive_power' modülünden 'ExecutiveAction' ve 'party' modülünden 'Party' yapıları kullanılıyor.
use super::{executive_power::ExecutiveAction, party::Party};
// Serde kütüphanesinden 'Deserialize' ve 'Serialize' trait'leri kullanılıyor.
use serde::{Deserialize, Serialize};

// 'Board' yapısı tanımlanıyor ve 'Clone', 'Serialize', 'Deserialize' ve 'Debug' trait'lerini implemente ediyor.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Board {
    // Board yapısının alanları tanımlanıyor.
    pub num_players: usize, // Oyuncu sayısı
    pub liberal_cards: usize, // Liberal kart sayısı
    pub fascist_cards: usize, // Faşist kart sayısı
    pub communist_cards: usize, // Komünist kart sayısı
}

// Board yapısına ait fonksiyonlar tanımlanıyor.
impl Board {
    /// Yeni bir board oluşturur.
    pub fn new(num_players: usize) -> Self {
        // Yeni bir Board yapısı döndürür, başlangıçta kart sayıları 0'dır.
        Board {
            num_players,
            liberal_cards: 0,
            fascist_cards: 0,
            communist_cards: 0,
        }
    }

    /// Bir politika kartı oynar.
    pub fn play_card(&mut self, party: Party) {
        // Parti türüne göre ilgili kart sayısını bir artırır.
        match party {
            Party::Liberal => self.liberal_cards += 1,
            Party::Fascist => self.fascist_cards += 1,
            Party::Communist => self.communist_cards += 1,
        }
    }

    /// Son oynanan faşist kartın kilidini açtığı yürütme eylemini döndürür, eğer varsa.
    pub fn get_executive_power(&self, party: Party) -> Option<ExecutiveAction> {
        use ExecutiveAction::*;
        // Parti türüne ve faşist/komünist kart sayısına göre yürütme eylemini belirler.
        match party {
            Party::Liberal => None, // Liberal parti için yürütme eylemi yok.
            Party::Fascist => match (self.num_players, self.fascist_cards) {
                (9..=10, 1) => Some(InvestigatePlayer),
                (7..=10, 2) => Some(InvestigatePlayer),
                (5..=6, 3) => Some(PolicyPeak),
                (7..=10, 3) => Some(SpecialElection),
                (_, 4) => Some(Execution),
                (_, 5) => Some(Execution),
                _ => None,
            },
            Party::Communist => match (self.num_players, self.communist_cards) {
                (_, 1) => Some(Bugging),
                (_, 2) => Some(Radicalisation),
                (_, 3) => Some(FiveYearPlan),
                (_, 4) => Some(Congress),
                (8.., 5) => Some(Confession),
                _ => None,
            },
        }
    }

    /// Oynanan kartın oyunu kazanıp kazanmadığını kontrol eder.
    pub fn is_winning_card(&self, party: Party) -> bool {
        // Parti türüne göre kalan maksimum kart sayısını kontrol eder.
        match party {
            Party::Liberal => self.liberal_cards == self.max_liberal_cards() - 1,
            Party::Fascist => self.fascist_cards == self.max_fascist_cards() - 1,
            Party::Communist => self.communist_cards == self.max_communist_cards() - 1,
        }
    }

    /// Herhangi bir partinin politika izini tamamlayıp tamamlamadığını kontrol eder.
    pub fn check_tracks(&self) -> Option<Party> {
        // Parti türüne göre maksimum kart sayısını kontrol eder ve kazanan partiyi döndürür.
        if self.liberal_cards == self.max_liberal_cards() {
            return Some(Party::Liberal);
        }
        if self.fascist_cards == self.max_fascist_cards() {
            return Some(Party::Fascist);
        }
        if self.communist_cards == self.max_communist_cards() {
            return Some(Party::Communist);
        }
        None
    }

    /// Veto yetkisinin kilidinin açılıp açılmadığını kontrol eder.
    pub fn veto_unlocked(&self) -> bool {
        // Faşist kart sayısı 5 veya daha fazla ise veto yetkisi açılır.
        self.fascist_cards >= 5
    }

    // Maksimum liberal kart sayısını döndürür.
    fn max_liberal_cards(&self) -> usize {
        5
    }

    // Maksimum faşist kart sayısını döndürür.
    fn max_fascist_cards(&self) -> usize {
        6
    }

    // Maksimum komünist kart sayısını döndürür.
    fn max_communist_cards(&self) -> usize {
        if self.num_players < 8 {
            5
        } else {
            6
        }
    }
}