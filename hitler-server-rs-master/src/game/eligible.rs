use super::{party::Party, player::Role, Game, MAX_PLAYERS}; // party, player ve Game modüllerini ve MAX_PLAYERS sabitini dahil et
use serde::{Deserialize, Serialize}; // Serde'nin Deserialize ve Serialize trait'lerini dahil et

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct EligiblePlayers {
    eligible: [bool; MAX_PLAYERS], // MAX_PLAYERS boyutunda bir boolean dizisi
}

impl EligiblePlayers {
    pub fn only_one(player: usize) -> Self {
        Self {
            eligible: core::array::from_fn(|i| i == player), // Sadece belirli bir oyuncuyu uygun yapar
        }
    }

    pub fn only(players: &[usize]) -> Self {
        Self {
            eligible: core::array::from_fn(|i| players.contains(&i)), // Sadece belirli oyuncuları uygun yapar
        }
    }

    pub fn exclude(&mut self, player: usize) {
        self.eligible[player] = false; // Belirli bir oyuncuyu uygunlar listesinden çıkarır
    }

    pub fn includes(&self, player: usize) -> bool {
        self.eligible[player] // Belirli bir oyuncunun uygun olup olmadığını kontrol eder
    }

    pub fn names(&self, game: &Game) -> Vec<String> {
        game.players
            .iter()
            .enumerate()
            .filter(|(i, _)| self.includes(*i)) // Uygun oyuncuları filtreler
            .map(|(_, p)| p.name.clone()) // Uygun oyuncuların isimlerini toplar
            .collect() // İsimleri bir vektörde toplar
    }
}

pub struct EligiblePlayersBuilder<'a> {
    game: &'a Game,
    eligible: [bool; MAX_PLAYERS], // MAX_PLAYERS boyutunda bir boolean dizisi
}

impl Game {
    pub fn eligible_players(&self) -> EligiblePlayersBuilder<'_> {
        EligiblePlayersBuilder {
            game: self,
            eligible: core::array::from_fn(|i| self.players.get(i).map(|p| p.alive).unwrap_or(false)), // Yaşayan oyuncuları uygun olarak işaretler
        }
    }
}

impl<'a> EligiblePlayersBuilder<'a> {
    pub fn exclude(mut self, player: usize) -> Self {
        self.eligible[player] = false; // Belirli bir oyuncuyu uygunlar listesinden çıkarır
        self
    }

    pub fn ordinary_communist(mut self) -> Self {
        for (idx, player) in self.game.players.iter().enumerate() {
            self.eligible[idx] &= player.role == Role::Communist; // Sadece komünist rolündeki oyuncuları uygun yapar
        }
        self
    }

    pub fn can_radicalise(mut self) -> Self {
        for (idx, player) in self.game.players.iter().enumerate() {
            self.eligible[idx] &= player.party() != Party::Communist; // Komünist olmayanları uygular
            self.eligible[idx] &= !player.investigated; // Araştırılmamış olanları uygular
            self.eligible[idx] &= !player.tried_to_radicalise; // Radikalleşmeye çalışmamış olanları uygular
        }
        self
    }

    pub fn not_investigated(mut self) -> Self {
        for (idx, player) in self.game.players.iter().enumerate() {
            self.eligible[idx] &= !player.investigated; // Araştırılmamış olanları uygular
        }
        self
    }

    pub fn make(self) -> EligiblePlayers {
        EligiblePlayers { eligible: self.eligible } // EligiblePlayers nesnesini oluşturur
    }
}