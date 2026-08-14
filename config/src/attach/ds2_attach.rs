use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ds2AttachConfig {
    pub no_death:                 bool,
    pub no_damage:                bool,
    pub infinite_poise:           bool,
    pub infinite_stamina:         bool,
    pub infinite_durability:      bool,
    pub infinite_consumables:     bool,
    pub no_hollowing:             bool,
    pub no_soul_loss:             bool,
    pub hidden:                   bool,
    pub silent:                   bool,
    pub skip_credits:             bool,
    pub fast_quitout:             bool,
    pub disable_roll:             bool,
    pub disable_backstep:         bool,
    pub skip_ivory_king_gauntlet: bool,
    pub disable_loyce_knights:    bool,
    pub start_event_logger:       bool,
}
