use super::{board::Board, party::Party}; // board ve party modüllerini dahil et
use rand::prelude::SliceRandom; // SliceRandom trait'ini dahil et
use rand::Rng; // Rng trait'ini dahil et
use serde::{Deserialize, Serialize}; // Serde'nin Deserialize ve Serialize trait'lerini dahil et
use std::iter::repeat; // repeat fonksiyonunu dahil et

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Deck {
    /// Deste, atık yığını ve oyun tahtasında toplam liberal kart sayısı
    liberal: usize,
    /// Deste, atık yığını ve oyun tahtasında toplam faşist kart sayısı
    fascist: usize,
    /// Deste, atık yığını ve oyun tahtasında toplam komünist kart sayısı
    communist: usize,
    /// Mevcut çekme destesi
    deck: Vec<Party>,
}

impl Deck {
    pub fn new(communists: bool) -> Self {
        let (liberal, fascist, communist) = match communists {
            false => (6, 11, 0), // Komünistler yoksa, 6 liberal, 11 faşist kart
            true => (6, 14, 8), // Komünistler varsa, 6 liberal, 14 faşist, 8 komünist kart
        };
        Self { liberal, fascist, communist, deck: vec![] }
    }

    /// Çekme destesinde üçten az kart varsa, atık yığını desteye karıştırır.
    pub fn check_shuffle(&mut self, board: &Board, rng: &mut impl Rng) {
        if self.deck.len() < 3 {
            self.shuffle(board, rng); // Eğer deste üçten az kart içeriyorsa, karıştır
        }
    }

    /// Atık yığını desteye karıştırır.
    pub fn shuffle(&mut self, board: &Board, rng: &mut impl Rng) {
        let liberal = self.liberal - board.liberal_cards; // Tahtadaki liberal kartları çıkar
        let fascist = self.fascist - board.fascist_cards; // Tahtadaki faşist kartları çıkar
        let communist = self.communist - board.communist_cards; // Tahtadaki komünist kartları çıkar

        self.deck.clear(); // Desteyi temizle
        self.deck.extend(repeat(Party::Liberal).take(liberal)); // Liberal kartları ekle
        self.deck.extend(repeat(Party::Fascist).take(fascist)); // Faşist kartları ekle
        self.deck.extend(repeat(Party::Communist).take(communist)); // Komünist kartları ekle
        self.deck.shuffle(rng); // Desteyi karıştır
    }

    /// Desteye iki komünist kart ve bir liberal kart karıştırır.
    pub fn five_year_plan(&mut self, rng: &mut impl Rng) {
        self.communist += 2; // İki komünist kart ekle
        self.liberal += 1; // Bir liberal kart ekle
        self.deck.push(Party::Communist); // Komünist kart ekle
        self.deck.push(Party::Communist); // Komünist kart ekle
        self.deck.push(Party::Liberal); // Liberal kart ekle
        self.deck.shuffle(rng); // Desteyi karıştır
    }

    /// Desteden bir kart çeker.
    pub fn draw_one(&mut self) -> Party {
        self.deck.pop().unwrap() // Üstteki kartı çek
    }

    /// Desteden üç kart çeker.
    pub fn draw_three(&mut self) -> [Party; 3] {
        let mut cards = [
            self.deck.pop().unwrap(), // İlk kart
            self.deck.pop().unwrap(), // İkinci kart
            self.deck.pop().unwrap(), // Üçüncü kart
        ];
        cards.reverse(); // Kartları ters çevir
        cards
    }

    /// Çekme destesindeki kartların sayısı.
    pub fn count(&self) -> usize {
        self.deck.len() // Destede kaç kart olduğunu döndür
    }

    /// Çekme destesinin üstteki üç kartına bakar.
    pub fn peek_three(&self) -> [Party; 3] {
        self.deck[self.deck.len() - 3..].try_into().unwrap() // Üstteki üç karta bak
    }
}