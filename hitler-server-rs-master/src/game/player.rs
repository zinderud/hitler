use super::{party::Party, GameOptions, MAX_PLAYERS}; // Diğer modüllerden Party, GameOptions ve MAX_PLAYERS'i kullan
use crate::error::GameError; // crate modülünden GameError'u kullan
use rand::prelude::SliceRandom; // rand kütüphanesinden SliceRandom'u kullan
use serde::{Deserialize, Serialize}; // serde kütüphanesinden Deserialize ve Serialize'i kullan
use std::iter::repeat; // std kütüphanesinden repeat fonksiyonunu kullan

/// Bir oyun oyuncusu.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String, // Oyuncunun adı
    pub role: Role, // Oyuncunun rolü
    pub others: [InvestigationResult; MAX_PLAYERS], // Diğer oyuncuların araştırma sonuçları
    pub alive: bool, // Oyuncunun hayatta olup olmadığı bilgisi
    pub not_hitler: bool, // Oyuncunun Hitler olup olmadığı bilgisi
    pub investigated: bool, // Oyuncunun araştırılıp araştırılmadığı bilgisi
    pub tried_to_radicalise: bool, // Oyuncunun radikalleşmeye çalışıp çalışmadığı bilgisi
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Role {
    Liberal, // Liberal rolü
    Fascist, // Faşist rolü
    Communist, // Komünist rolü
    Hitler, // Hitler rolü
    Monarchist, // Monarşist rolü
    Anarchist, // Anarşist rolü
    Capitalist, // Kapitalist rolü
    Centrist, // Merkezci rolü
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Liberal => "Liberal",
            Role::Fascist => "Fascist",
            Role::Communist => "Communist",
            Role::Hitler => "Hitler",
            Role::Monarchist => "Monarchist",
            Role::Anarchist => "Anarchist",
            Role::Capitalist => "Capitalist",
            Role::Centrist => "Centrist",
        }
        .to_string()
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum InvestigationResult {
    Unknown, // Bilinmeyen araştırma sonucu
    Party(Party), // Parti araştırma sonucu
    Role(Role), // Rol araştırma sonucu
}

impl Player {
    pub fn new(name: String, role: Role) -> Self {
        Self {
            name, // Oyuncunun adı
            role, // Oyuncunun rolü
            others: [InvestigationResult::Unknown; MAX_PLAYERS], // Diğer oyuncuların araştırma sonuçları (bilinmiyor olarak başlatılır)
            alive: true, // Oyuncu hayatta olarak başlatılır
            not_hitler: false, // Oyuncu Hitler olarak başlatılmaz
            investigated: false, // Oyuncu araştırılmamış olarak başlatılır
            tried_to_radicalise: false, // Oyuncu radikalleşmeye çalışmamış olarak başlatılır
        }
    }

    pub fn party(&self) -> Party {
        match self.role {
            Role::Liberal => Party::Liberal, // Eğer rol Liberal ise parti Liberal
            Role::Fascist => Party::Fascist, // Eğer rol Faşist ise parti Faşist
            Role::Communist => Party::Communist, // Eğer rol Komünist ise parti Komünist
            Role::Hitler => Party::Fascist, // Eğer rol Hitler ise parti Faşist
            Role::Monarchist => Party::Fascist, // Eğer rol Monarşist ise parti Faşist
            Role::Anarchist => Party::Communist, // Eğer rol Anarşist ise parti Komünist
            Role::Capitalist => Party::Liberal, // Eğer rol Kapitalist ise parti Liberal
            Role::Centrist => Party::Liberal, // Eğer rol Merkezci ise parti Liberal
        }
    }

    pub fn radicalise(&mut self) -> bool {
        self.tried_to_radicalise = true; // Oyuncu radikalleşmeye çalıştı olarak işaretlenir
        if matches!(self.role, Role::Liberal | Role::Centrist) { // Eğer rol Liberal veya Merkezci ise
            self.role = Role::Communist; // Rol Komünist olarak değiştirilir
            true // Radikalleşme başarılı
        } else {
            false // Radikalleşme başarısız
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerDistribution {
    pub num_players: usize, // Oyuncu sayısı
    pub liberals: usize, // Liberal oyuncu sayısı
    pub fascists: usize, // Faşist oyuncu sayısı
    pub communists: usize, // Komünist oyuncu sayısı
    pub hitler: bool, // Hitler rolü olup olmadığı
    pub monarchist: bool, // Monarşist rolü olup olmadığı
    pub anarchist: bool, // Anarşist rolü olup olmadığı
    pub capitalist: bool, // Kapitalist rolü olup olmadığı
    pub centrists: bool, // Merkezci rolü olup olmadığı
}

impl PlayerDistribution {
    pub fn new(opts: &GameOptions, num_players: usize) -> Result<Self, GameError> {
        let mut fascists: isize; // Faşist oyuncu sayısı
        let mut communists: isize; // Komünist oyuncu sayısı
        let mut liberals: isize; // Liberal oyuncu sayısı

        // Her parti için oyuncu sayısını hesapla
        if opts.communists {
            fascists = match num_players {
                ..=5 => return Err(GameError::TooFewPlayers), // Eğer oyuncu sayısı 5 veya daha az ise hata döndür
                6..=7 => 2, // 6-7 oyuncu için 2 faşist
                8..=10 => 3, // 8-10 oyuncu için 3 faşist
                11..=14 => 4, // 11-14 oyuncu için 4 faşist
                15..=16 => 5, // 15-16 oyuncu için 5 faşist
                _ => return Err(GameError::TooManyPlayers), // Eğer oyuncu sayısı 16'dan fazla ise hata döndür
            };
            communists = match num_players {
                ..=5 => return Err(GameError::TooFewPlayers), // Eğer oyuncu sayısı 5 veya daha az ise hata döndür
                6..=8 => 1, // 6-8 oyuncu için 1 komünist
                9..=12 => 2, // 9-12 oyuncu için 2 komünist
                13..=15 => 3, // 13-15 oyuncu için 3 komünist
                16 => 4, // 16 oyuncu için 4 komünist
                _ => return Err(GameError::TooManyPlayers), // Eğer oyuncu sayısı 16'dan fazla ise hata döndür
            };
        } else {
            fascists = match num_players {
                ..=4 => return Err(GameError::TooFewPlayers), // Eğer oyuncu sayısı 4 veya daha az ise hata döndür
                5..=10 => (num_players as isize - 1) / 2, // 5-10 oyuncu için faşist sayısı (oyuncu sayısı - 1) / 2
                _ => return Err(GameError::TooManyPlayers), // Eğer oyuncu sayısı 10'dan fazla ise hata döndür
            };
            communists = 0; // Komünist sayısı 0
        }
        liberals = num_players as isize - (fascists + communists); // Liberal sayısını hesapla

        // Özel rolleri çıkar
        let hitler = true; // Hitler rolü var
        let GameOptions {
            monarchist, anarchist, capitalist, centrists, .. // Oyun seçeneklerinden özel rolleri al
        } = *opts;

        fascists -= hitler as isize; // Hitler rolünü faşistlerden çıkar
        fascists -= monarchist as isize; // Monarşist rolünü faşistlerden çıkar
        communists -= anarchist as isize; // Anarşist rolünü komünistlerden çıkar
        liberals -= capitalist as isize; // Kapitalist rolünü liberallerden çıkar
        liberals -= 2 * (centrists as isize); // Merkezci rolünü liberallerden çıkar (iki kez)

        // Yeterli sayıda "sıradan" oyuncu olduğundan emin ol
        let min_communists = opts.communists as isize; // Minimum komünist sayısını belirle
        if fascists < 1 || communists < min_communists || liberals < 0 {
            return Err(GameError::TooFewPlayers); // Eğer yeterli oyuncu yoksa hata döndür
        }

        // Sonucu döndür
        Ok(Self {
            num_players, // Oyuncu sayısı
            liberals: liberals as usize, // Liberal oyuncu sayısı
            fascists: fascists as usize, // Faşist oyuncu sayısı
            communists: communists as usize, // Komünist oyuncu sayısı
            hitler, // Hitler rolü olup olmadığı
            monarchist, // Monarşist rolü olup olmadığı
            anarchist, // Anarşist rolü olup olmadığı
            capitalist, // Kapitalist rolü olup olmadığı
            centrists, // Merkezci rolü olup olmadığı
        })
    }
}

pub fn assign_roles(distr: PlayerDistribution, rng: &mut impl rand::Rng) -> Vec<Role> {
    let mut roles = Vec::with_capacity(distr.num_players); // Oyuncu sayısı kadar kapasiteye sahip bir rol vektörü oluştur

    roles.extend(repeat(Role::Fascist).take(distr.fascists)); // Faşist rollerini ekle
    roles.extend(repeat(Role::Communist).take(distr.communists)); // Komünist rollerini ekle
    roles.extend(repeat(Role::Liberal).take(distr.liberals)); // Liberal rollerini ekle

    if distr.hitler {
        roles.push(Role::Hitler); // Eğer Hitler rolü varsa ekle
    }
    if distr.monarchist {
        roles.push(Role::Monarchist); // Eğer Monarşist rolü varsa ekle
    }
    if distr.anarchist {
        roles.push(Role::Anarchist); // Eğer Anarşist rolü varsa ekle
    }
    if distr.capitalist {
        roles.push(Role::Capitalist); // Eğer Kapitalist rolü varsa ekle
    }
    if distr.centrists {
        roles.push(Role::Centrist); // Eğer Merkezci rolü varsa ekle
        roles.push(Role::Centrist); // İkinci Merkezci rolünü ekle
    }

    assert_eq!(roles.len(), distr.num_players); // Roller ve oyuncu sayısının eşit olduğunu doğrula

    roles.shuffle(rng); // Rolleri karıştır
    roles // Rolleri döndür
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::player::Role;
    use crate::game::GameOptions;

    #[test]
    fn role_assignment_10players() {
        // 2+H, 5, 2
        let opts = GameOptions {
            communists: true,
            anarchist: true,
            capitalist: true,
            centrists: true,
            monarchist: false,
        };
        let distr = PlayerDistribution::new(&opts, 10).unwrap();
        println!("{:?}", &distr);
        let roles = assign_roles(distr, &mut rand::thread_rng());
        assert_eq!(roles.iter().filter(|r| **r == Role::Hitler).count(), 1); // Hitler rolü sayısını doğrula
        assert_eq!(roles.iter().filter(|r| **r == Role::Monarchist).count(), 0); // Monarşist rolü sayısını doğrula
        assert_eq!(roles.iter().filter(|r| **r == Role::Fascist).count(), 2); // Faşist rolü sayısını doğrula
        assert_eq!(roles.iter().filter(|r| **r == Role::Capitalist).count(), 1); // Kapitalist rolü sayısını doğrula
        assert_eq!(roles.iter().filter(|r| **r == Role::Centrist).count(), 2); // Merkezci rolü sayısını doğrula
        assert_eq!(roles.iter().filter(|r| **r == Role::Liberal).count(), 2); // Liberal rolü sayısını doğrula
        assert_eq!(roles.iter().filter(|r| **r == Role::Anarchist).count(), 1); // Anarşist rolü sayısını doğrula
        assert_eq!(roles.iter().filter(|r| **r == Role::Communist).count(), 1); // Komünist rolü sayısını doğrula
    }
}