use serde::{Deserialize, Serialize}; // Serde kütüphanesinden Deserialize ve Serialize trait'lerini dahil et

/// Oyunun iki siyasi partisi.
/// The two political parties of the game.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Party {
    Liberal, // Liberal parti
    Fascist, // Faşist parti
    Communist, // Komünist parti
}

impl ToString for Party {
    fn to_string(&self) -> String {
        match self {
            Party::Liberal => "Liberal", // Liberal partiyi "Liberal" olarak dönüştür
            Party::Fascist => "Fascist", // Faşist partiyi "Fascist" olarak dönüştür
            Party::Communist => "Communist", // Komünist partiyi "Communist" olarak dönüştür
        }
        .to_string()
    }
}

/* Original Code:
use serde::{Deserialize, Serialize};

/// The two political parties of the game.
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Party {
    Liberal,
    Fascist,
    Communist,
}

impl ToString for Party {
    fn to_string(&self) -> String {
        match self {
            Party::Liberal => "Liberal",
            Party::Fascist => "Fascist",
            Party::Communist => "Communist",
        }
        .to_string()
    }
}
*/