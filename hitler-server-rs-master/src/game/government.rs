use serde::{Deserialize, Serialize}; // Serde kütüphanesinden Deserialize ve Serialize trait'lerini dahil et

#[derive(Clone, Copy, Serialize, Deserialize, Debug)] // Government yapısının çeşitli özelliklerini türet
pub struct Government {
    pub president: usize, // Başkanın indeksini tutan bir alan
    pub chancellor: usize, // Şansölyenin indeksini tutan bir alan
}