use super::{player::Role, Game, GameState, NextPresident}; // player, Game, GameState ve NextPresident modüllerini dahil et
use crate::{
    error::GameError, // GameError modülünü dahil et
    game::{confirmations::Confirmations, eligible::EligiblePlayers, government::Government}, // game modülünün confirmations, eligible ve government alt modüllerini dahil et
};
use serde::{Deserialize, Serialize}; // Serde'nin Deserialize ve Serialize trait'lerini dahil et

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum ExecutiveAction {
    /// Başkan bir oyuncunun sadakatini araştırmak zorundadır.
    InvestigatePlayer,
    /// Başkan özel bir seçim çağrısı yapmak zorundadır.
    SpecialElection,
    /// Başkan destedeki ilk üç kartı görmek zorundadır.
    PolicyPeak,
    /// Başkan bir oyuncuyu idam etmek zorundadır.
    Execution,
    /// Komünistler bir oyuncunun parti üyeliğini öğrenir.
    Bugging,
    /// Komünistler bir oyuncuyu komünist ekibe dönüştürmeye çalışır.
    Radicalisation,
    /// 2 komünist ve 1 liberal politika desteye karıştırılır.
    FiveYearPlan,
    /// Yeni komünist müttefiklerini öğrenir veya başka bir radikalleşme denemesi yapılır.
    Congress,
    /// Başkan veya şansölye partilerini açıklar.
    Confession,
}

impl ToString for ExecutiveAction {
    fn to_string(&self) -> String {
        match self {
            ExecutiveAction::InvestigatePlayer => "investigate",
            ExecutiveAction::SpecialElection { .. } => "specialElection",
            ExecutiveAction::PolicyPeak => "policyPeak",
            ExecutiveAction::Execution => "execution",
            ExecutiveAction::Bugging => "bugging",
            ExecutiveAction::Radicalisation => "radicalisation",
            ExecutiveAction::FiveYearPlan => "fiveYearPlan",
            ExecutiveAction::Congress => "congress",
            ExecutiveAction::Confession => "confession",
        }
        .to_string()
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum ConfessionChoice {
    President,
    Chancellor,
}

impl Game {
    /// Bir icra eylemi başlatır.
    pub fn start_executive_action(&mut self, action: ExecutiveAction) {
        use ExecutiveAction::*;

        // Bir icra gücü oynanabilmesi için son bir hükümet olmalıdır
        let Government { president, chancellor } = self.last_government.unwrap();

        match action {
            InvestigatePlayer => {
                self.state = GameState::ChoosePlayer {
                    action, // Eylem türünü belirle
                    can_select: EligiblePlayers::only_one(president), // Sadece başkan seçebilir
                    can_be_selected: self.eligible_players().not_investigated().exclude(president).make(), // Başkan hariç araştırılmamış olanlar seçilebilir
                };
            }
            SpecialElection => {
                // Not: Diğer oyunculara bilgi vermemek için, ölü olsalar bile "monarşisti beklemek" istiyoruz.
                let monarchist = self.players.iter().position(|p| p.role == Role::Monarchist); // Monarşisti bul
                if let Some(monarchist) = monarchist {
                    self.state = GameState::PromptMonarchist {
                        monarchist, // Monarşist pozisyonu
                        last_president: president, // Son başkan
                        hijacked: false, // Kaçırılmadı
                    };
                } else {
                    self.state = GameState::ChoosePlayer {
                        action,
                        can_select: EligiblePlayers::only_one(president), // Sadece başkan seçebilir
                        can_be_selected: self.eligible_players().exclude(president).make(), // Başkan hariç herkes seçilebilir
                    };
                }
            }
            Execution => {
                self.state = GameState::ChoosePlayer {
                    action,
                    can_select: EligiblePlayers::only_one(president), // Sadece başkan seçebilir
                    can_be_selected: self.eligible_players().exclude(president).make(), // Başkan hariç herkes seçilebilir
                };
            }
            PolicyPeak | FiveYearPlan => {
                self.state = GameState::ActionReveal {
                    action,
                    chosen_player: None, // Seçilen oyuncu yok
                    confirmations: Confirmations::new(self.num_players_alive()), // Canlı oyuncu sayısına göre onaylar
                };
            }
            Bugging | Radicalisation | Congress => {
                self.state = GameState::CommunistStart { action }; // Komünist başlangıç durumu
            }
            Confession => {
                self.state = GameState::ChoosePlayer {
                    action,
                    can_select: EligiblePlayers::only_one(chancellor), // Sadece şansölye seçebilir
                    can_be_selected: EligiblePlayers::only(&[president, chancellor]), // Sadece başkan ve şansölye seçilebilir
                };
            }
        }
    }

    /// "Komünist oturum" başladığında çağrılır.
    pub fn end_communist_start(&mut self) -> Result<(), GameError> {
        use ExecutiveAction::*;

        let GameState::CommunistStart { action } = self.state else {
            return Err(GameError::InvalidAction);
        };

        // Radikalleşme başarılı olduysa, kongre sırasında ikinci bir deneme yapılmaz
        if action == Congress && self.radicalised {
            self.state = GameState::Congress;
            return Ok(());
        }

        let can_select = self.eligible_players().ordinary_communist().make(); // Sadece sıradan komünistler seçebilir

        let mut can_be_selected = self.eligible_players().can_radicalise(); // Radikalleşebilecek oyuncular
        if matches!(action, Radicalisation | Congress) {
            can_be_selected = can_be_selected.not_investigated(); // Araştırılmamış olanlar
        }
        let can_be_selected = can_be_selected.make(); // Seçilebilir oyuncular

        self.state = GameState::ChoosePlayer { action, can_select, can_be_selected };
        Ok(())
    }

    /// Bir oyuncu kongre oturumunu bitirmeye hazır olduğunda çağrılır.
    pub fn end_congress(&mut self, player: usize) -> Result<(), GameError> {
        let GameState::Congress = &self.state else {
            return Err(GameError::InvalidAction);
        };
        if self.players.get(player).map(|p| p.role) != Some(Role::Communist) {
            return Err(GameError::InvalidAction);
        }
        self.state = GameState::CommunistEnd {
            action: ExecutiveAction::Congress,
            chosen_player: None,
        };
        Ok(())
    }

    /// Monarşist özel bir seçimi kaçırmayı seçtiğinde çağrılır.
    pub fn hijack_special_election(&mut self, player: usize) -> Result<(), GameError> {
        let GameState::PromptMonarchist { monarchist, hijacked, .. } = &mut self.state else {
            return Err(GameError::InvalidAction);
        };

        if player != *monarchist || !self.players[player].alive {
            return Err(GameError::InvalidAction);
        };

        *hijacked = true;
        Ok(())
    }

    /// Tahta özel seçim ekranını sunmayı bitirdiğinde çağrılır.
    pub fn start_special_election(&mut self) -> Result<(), GameError> {
        let GameState::PromptMonarchist { monarchist, last_president, hijacked } = self.state else {
            return Err(GameError::InvalidAction);
        };

        if hijacked {
            self.next_president = Some(NextPresident::Monarchist { monarchist, last_president });
            self.start_round();
        } else {
            self.state = GameState::ChoosePlayer {
                action: ExecutiveAction::SpecialElection,
                can_select: EligiblePlayers::only_one(last_president), // Sadece son başkan seçebilir
                can_be_selected: self.eligible_players().exclude(last_president).make(), // Son başkan hariç herkes seçilebilir
            };
        }
        Ok(())
    }

    /// "Komünist oturum" bittiğinde çağrılır.
    pub fn end_communist_end(&mut self) -> Result<(), GameError> {
        use ExecutiveAction::*;

        let GameState::CommunistEnd { action, chosen_player } = self.state else {
            return Err(GameError::InvalidAction);
        };

        match action {
            Bugging => {
                self.start_round();
            }
            Radicalisation | Congress => {
                if let Some(player_idx) = chosen_player {
                    let player = &mut self.players[player_idx];
                    self.radicalised = player.radicalise();
                }
                self.state = GameState::ActionReveal {
                    action,
                    chosen_player,
                    confirmations: Confirmations::new(self.num_players_alive()),
                };
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    /// Tahta icra eylemini sunmayı bitirdiğinde çağrılır.
    pub fn end_executive_action(&mut self, player: Option<usize>) -> Result<(), GameError> {
        use ExecutiveAction::*;

        let GameState::ActionReveal { action, chosen_player, confirmations } = &mut self.state else {
            return Err(GameError::InvalidAction);
        };

        match action {
            // Bu eylemleri sadece başkan bitirebilir
            InvestigatePlayer | PolicyPeak => {
                let president = self.last_government.unwrap().president;
                if player != Some(president) {
                    return Err(GameError::InvalidAction);
                }
            }
            // Bu eylemleri sadece tahta bitirebilir
            SpecialElection | Execution | FiveYearPlan | Confession => {
                if player.is_some() {
                    return Err(GameError::InvalidAction);
                }
            }
            // Bu eylemler, tüm oyuncular hazır olduğunda sona erer
            Bugging | Radicalisation | Congress => {
                let Some(player) = player else {
                    return Err(GameError::InvalidAction);
                };
                confirmations.confirm(player);
                if !confirmations.can_proceed() {
                    return Ok(());
                }
            }
        };

        match action {
            InvestigatePlayer => {
                self.players[chosen_player.unwrap()].investigated = true; // Oyuncu araştırıldı olarak işaretlenir
                self.start_round();
            }
            SpecialElection => {
                let player = chosen_player.unwrap();
                self.next_president = Some(NextPresident::Normal { player }); // Yeni başkan olarak atanır
                self.start_round();
            }
            Execution => {
                let player = &mut self.players[chosen_player.unwrap()];
                player.alive = false; // Oyuncu ölü olarak işaretlenir
                player.not_hitler = player.role != Role::Hitler; // Oyuncu Hitler değilse işaretlenir

                if self.check_game_over() { // Oyun bitti mi kontrol edilir
                    return Ok(());
                }

                self.start_round();
            }
            Bugging => {
                self.state = GameState::CommunistEnd { action: *action, chosen_player: None }; // Komünist bitiş durumu
            }
            FiveYearPlan => {
                self.deck.five_year_plan(&mut self.rng); // Beş Yıllık Plan desteye eklenir
                self.start_round();
            }
            _ => {
                self.start_round();
            }
        }
        Ok(())
    }
}