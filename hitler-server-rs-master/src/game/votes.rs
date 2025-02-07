use serde::{Deserialize, Serialize};
// Serde kütüphanesini kullanarak veri serileştirme ve seriden çıkarma işlemleri yapıyoruz.

use super::MAX_PLAYERS;
// Üst modülden MAX_PLAYERS sabitini kullanıyoruz.

/// Her bir oyuncunun oyunu takip eder.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Votes {
    num_players: usize, // Oyuncu sayısı.
    votes: [Option<bool>; MAX_PLAYERS], // Her bir oyuncunun oyunu tutan dizi.
}

impl Votes {
    /// Yeni bir `Votes` oluşturur.
    pub fn new(num_players: usize) -> Self {
        let votes = [None; MAX_PLAYERS]; // Oyları None olarak başlat.
        Self { num_players, votes }
    }

    /// Verilen oyuncunun oy kullanıp kullanmadığını döndürür.
    pub fn has_cast(&self, player_idx: usize) -> bool {
        self.votes[player_idx].is_some()
    }

    /// Bir oyuncunun oyunu kaydeder.
    pub fn vote(&mut self, player_idx: usize, vote: bool) {
        self.votes[player_idx] = Some(vote);
    }

    /// Tüm oylar sayıldıysa sonucu döndürür, aksi takdirde `None` döner.
    pub fn outcome(&self) -> Option<bool> {
        let yes = self.votes.iter().filter(|v| **v == Some(true)).count(); // Evet oylarını say.
        let no = self.votes.iter().filter(|v| **v == Some(false)).count(); // Hayır oylarını say.
        if std::env::var("QUICK_MODE").is_ok() {
            (yes + no > 0).then_some(yes > no) // QUICK_MODE aktifse, en az bir oy varsa sonucu kontrol et.
        } else {
            (yes + no >= self.num_players).then_some(yes > no) // QUICK_MODE aktif değilse, tüm oylar varsa sonucu kontrol et.
        }
    }

    /// Her bir oyuncunun oylarını döner.
    pub fn votes(&self) -> &[Option<bool>] {
        &self.votes
    }
}

/// Monarşist bir seçim sırasında her bir oyuncunun oyunu takip eder.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct MonarchistVotes {
    num_players: usize, // Oyuncu sayısı.
    /// Monarşist olan oyuncunun indeksi
    monarchist: usize,
    /// `true` monarşistin şansölyesine verilen oy, `false` diğerine verilen oy.
    votes: [Option<bool>; MAX_PLAYERS], // Oyları tutan dizi.
}

impl MonarchistVotes {
    /// Yeni bir `MonarchistVotes` oluşturur.
    pub fn new(num_players: usize, monarchist: usize) -> Self {
        let votes = [None; MAX_PLAYERS]; // Oyları None olarak başlat.
        Self { num_players, monarchist, votes }
    }

    /// Verilen oyuncunun oy kullanıp kullanmadığını döndürür.
    pub fn has_cast(&self, player_idx: usize) -> bool {
        self.votes[player_idx].is_some()
    }

    /// Bir oyuncunun oyunu kaydeder, `true` monarşistin seçimini belirtir.
    pub fn vote(&mut self, player_idx: usize, vote: bool) {
        self.votes[player_idx] = Some(vote);
    }

    /// Tüm oylar sayıldıysa sonucu döndürür, aksi takdirde `None` döner.
    /// `true` sonucu monarşistin seçiminin kazandığını belirtir.
    pub fn outcome(&self) -> Option<bool> {
        use std::cmp::Ordering::*;
        let yes = self.votes.iter().filter(|v| **v == Some(true)).count(); // Evet oylarını say.
        let no = self.votes.iter().filter(|v| **v == Some(false)).count(); // Hayır oylarını say.
        if std::env::var("QUICK_MODE").is_ok() {
            (yes + no > 0).then_some(yes > no) // QUICK_MODE aktifse, en az bir oy varsa sonucu kontrol et.
        } else {
            (yes + no >= self.num_players).then(|| match yes.cmp(&no) {
                Less => false, // Hayır oyları daha fazla ise sonuç false.
                Greater => true, // Evet oyları daha fazla ise sonuç true.
                Equal => self.votes[self.monarchist].unwrap_or(true), // Eşitse, monarşistin oyu belirleyici.
            })
        }
    }

    /// Her bir oyuncunun oylarını döner.
    pub fn votes(&self) -> &[Option<bool>] {
        &self.votes
    }
}