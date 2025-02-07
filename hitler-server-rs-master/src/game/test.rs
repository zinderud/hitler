#![cfg(test)] // Yalnızca test modunda bu kodun derlenmesini sağlar
#![allow(clippy::bool_assert_comparison)] // clippy lint'lerini bu dosyada devre dışı bırakır
use super::confirmations::Confirmations; // Confirmations modülünü kullanır
use super::player::Player; // Player modülünü kullanır
use super::player::Role; // Role modülünü kullanır
use super::GameState; // GameState modülünü kullanır
use super::Party::*; // Party alt modüllerini kullanır
use crate::game::deck::Deck; // Deck modülünü kullanır
use crate::game::government::Government; // Government modülünü kullanır
use crate::game::Game; // Game modülünü kullanır
use crate::game::GameOptions; // GameOptions modülünü kullanır
use crate::game::WinCondition; // WinCondition modülünü kullanır
use rand::SeedableRng; // SeedableRng modülünü kullanır
use rand_chacha::ChaCha8Rng; // ChaCha8Rng modülünü kullanır
#[test] // Bu fonksiyonun bir test fonksiyonu olduğunu belirtir
fn can_create_game() { // can_create_game adlı test fonksiyonunu tanımlar
   let players = ["Alex", "Bob", "Charlie", "David", "Ed"].map(|s| s.into()); // Oyuncu isimlerini belirler ve bunları Player objesine dönüştürür
   let opts = GameOptions::default(); // Varsayılan oyun seçeneklerini oluşturur
   let game = Game::new(opts, &players, 0).unwrap(); // Yeni bir oyun oluşturur
   assert!(matches!(game.state, GameState::Night { .. })); // Oyun durumunun 'Night' olduğunu doğrular
}
#[test] // Bu fonksiyonun bir test fonksiyonu olduğunu belirtir
fn liberal_track_victory() { // liberal_track_victory adlı test fonksiyonunu tanımlar
   let mut game = Game { // Yeni bir oyun durumu oluşturur
       opts: GameOptions::default(), // Varsayılan oyun seçeneklerini kullanır
       board: super::board::Board { // Oyun tahtasını tanımlar
           num_players: 5, // 5 oyuncu belirler
           liberal_cards: 4, // 4 liberal kart belirler
           fascist_cards: 0, // 0 faşist kart belirler
           communist_cards: 0, // 0 komünist kart belirler
       },
       deck: Deck::new(false), // Yeni bir deste oluşturur
       election_tracker: 0, // Seçim izleyici sayacını sıfırlar
       last_government: None, // Son hükümeti belirtmez
       players: vec![ // Oyuncuları tanımlar
           Player::new("ALEX".to_string(), Role::Liberal), // Liberal rolünde Alex oyuncusunu oluşturur
           Player::new("BOB".to_string(), Role::Liberal), // Liberal rolünde Bob oyuncusunu oluşturur
           Player::new("CHARLIE".to_string(), Role::Liberal), // Liberal rolünde Charlie oyuncusunu oluşturur
           Player::new("DAVID".to_string(), Role::Liberal), // Liberal rolünde David oyuncusunu oluşturur
           Player::new("ED".to_string(), Role::Liberal), // Liberal rolünde Ed oyuncusunu oluşturur
       ],
       presidential_turn: 0, // Başkanlık sırasını sıfırlar
       next_president: None, // Bir sonraki başkanı belirtmez
       rng: ChaCha8Rng::seed_from_u64(0), // Rastgele sayı üreteci oluşturur
       state: GameState::CardReveal { // Kart açma durumunu tanımlar
           result: Liberal, // Sonucu liberal olarak belirler
           chaos: false, // Kaos durumunu belirtmez
           confirmations: Confirmations::new(5), // 5 oyuncu için onaylar oluşturur
           board_ready: false, // Tahtanın hazır olmadığını belirtir
       },
       radicalised: false, // Radikalleşme durumunu belirtmez
       assassination: crate::game::AssassinationState::Unused, // Suikast durumunu belirtmez
   };
   game.end_card_reveal(None).unwrap(); // Kart açma durumunu sona erdirir
   assert!(game.game_over()); // Oyunun bittiğini doğrular
   assert!(matches!( // Oyun durumunun 'LiberalPolicyTrack' olduğunu doğrular
       game.state,
       GameState::GameOver(WinCondition::LiberalPolicyTrack)
   ));
}
#[test] // Bu fonksiyonun bir test fonksiyonu olduğunu belirtir
fn fascist_track_victory() { // fascist_track_victory adlı test fonksiyonunu tanımlar
   let mut game = Game { // Yeni bir oyun durumu oluşturur
       opts: GameOptions::default(), // Varsayılan oyun seçeneklerini kullanır
       board: super::board::Board { // Oyun tahtasını tanımlar
           num_players: 5, // 5 oyuncu belirler
           liberal_cards: 0, // 0 liberal kart belirler
           fascist_cards: 5, // 5 faşist kart belirler
           communist_cards: 0, // 0 komünist kart belirler
       },
       deck: Deck::new(false), // Yeni bir deste oluşturur
       election_tracker: 0, // Seçim izleyici sayacını sıfırlar
       last_government: None, // Son hükümeti belirtmez
       players: vec![ // Oyuncuları tanımlar
           Player::new("ALEX".to_string(), Role::Liberal), // Liberal rolünde Alex oyuncusunu oluşturur
           Player::new("BOB".to_string(), Role::Liberal), // Liberal rolünde Bob oyuncusunu oluşturur
           Player::new("CHARLIE".to_string(), Role::Liberal), // Liberal rolünde Charlie oyuncusunu oluşturur
           Player::new("DAVID".to_string(), Role::Liberal), // Liberal rolünde David oyuncusunu oluşturur
           Player::new("ED".to_string(), Role::Liberal), // Liberal rolünde Ed oyuncusunu oluşturur
       ],
       presidential_turn: 0, // Başkanlık sırasını sıfırlar
       next_president: None, // Bir sonraki başkanı belirtmez
       rng: ChaCha8Rng::seed_from_u64(0), // Rastgele sayı üreteci oluşturur
       state: GameState::CardReveal { // Kart açma durumunu tanımlar
           result: Fascist, // Sonucu faşist olarak belirler
           chaos: false, // Kaos durumunu belirtmez
           confirmations: Confirmations::new(5), // 5 oyuncu için onaylar oluşturur
           board_ready: false, // Tahtanın hazır olmadığını belirtir
       },
       radicalised: false, // Radikalleşme durumunu belirtmez
       assassination: crate::game::AssassinationState::Unused, // Suikast durumunu belirtmez
   };
    game.end_card_reveal(None).unwrap(); // Kart açma durumunu sona erdirir
    assert!(game.game_over()); // Oyunun bittiğini doğrular
    assert!(matches!( // Oyun durumunun 'FascistPolicyTrack' olduğunu doğrular
        game.state,
        GameState::GameOver(WinCondition::FascistPolicyTrack)
    ));
}
#[test] // Bu fonksiyonun bir test fonksiyonu olduğunu belirtir
fn eligible_chancellors_5players() { // eligible_chancellors_5players adlı test fonksiyonunu tanımlar
    let mut game = Game { // Yeni bir oyun durumu oluşturur
        opts: GameOptions::default(), // Varsayılan oyun seçeneklerini kullanır
        board: super::board::Board { // Oyun tahtasını tanımlar
            num_players: 5, // 5 oyuncu belirler
            liberal_cards: 0, // 0 liberal kart belirler
            fascist_cards: 0, // 0 faşist kart belirler
            communist_cards: 0, // 0 komünist kart belirler
        },
        deck: Deck::new(false), // Yeni bir deste oluşturur
        election_tracker: 0, // Seçim izleyici sayacını sıfırlar
        last_government: Some(Government { president: 0, chancellor: 3 }), // Son hükümeti belirler
        players: vec![ // Oyuncuları tanımlar
            Player::new("ALEX".to_string(), Role::Liberal), // Liberal rolünde Alex oyuncusunu oluşturur
            Player::new("BOB".to_string(), Role::Liberal), // Liberal rolünde Bob oyuncusunu oluşturur
            Player::new("CHARLIE".to_string(), Role::Liberal), // Liberal rolünde Charlie oyuncusunu oluşturur
            Player::new("DAVID".to_string(), Role::Fascist), // Faşist rolünde David oyuncusunu oluşturur
            Player::new("ED".to_string(), Role::Hitler), // Hitler rolünde Ed oyuncusunu oluşturur
        ],
        presidential_turn: 0, // Başkanlık sırasını sıfırlar
        next_president: None, // Bir sonraki başkanı belirtmez
        rng: ChaCha8Rng::seed_from_u64(0), // Rastgele sayı üreteci oluşturur
        state: GameState::CardReveal { // Kart açma durumunu tanımlar
            result: Fascist, // Sonucu faşist olarak belirler
            chaos: false, // Kaos durumunu belirtmez
            confirmations: Confirmations::new(5), // 5 oyuncu için onaylar oluşturur
            board_ready: false, // Tahtanın hazır olmadığını belirtir
        },
        radicalised: false, // Radikalleşme durumunu belirtmez
        assassination: crate::game::AssassinationState::Unused, // Suikast durumunu belirtmez
    };
    for i in 0..5 { // 0'dan 5'e kadar döngü oluşturur
        game.end_card_reveal(Some(i)).unwrap(); // Her bir kart açma durumunu sona erdirir
    }
    game.end_card_reveal(None).unwrap(); // Kart açma durumunu sona erdirir
    let GameState::Election { // Seçim durumunu tanımlar
        president, // Başkan
        chancellor, // Şansölye
        eligible_chancellors, // Uygun şansölyeler
        votes, // Oylar
    } = game.state // Oyun durumunu kontrol eder
    else { // Eğer değilse
        panic!("Expected an election"); // Panik durumu oluşturur
    };
    assert_eq!(president, 1); // Başkanın 1 olduğunu doğrular
    assert_eq!(chancellor, None); // Şansölyenin olmadığını doğrular
    assert_eq!(eligible_chancellors.includes(0), true); // 0'ın uygun olduğunu doğrular
    assert_eq!(eligible_chancellors.includes(1), false); // 1'in uygun olmadığını doğrular
    assert_eq!(eligible_chancellors.includes(2), true); // 2'nin uygun olduğunu doğrular
    assert_eq!(eligible_chancellors.includes(3), false); // 3'ün uygun olmadığını doğrular
    assert_eq!(eligible_chancellors.includes(4), true); // 4'ün uygun olduğunu doğrular
    assert_eq!(votes.outcome(), None); // Oy sonucunun olmadığını doğrular
}