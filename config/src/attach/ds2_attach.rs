use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Ds2AttachConfig {
    pub NoDeath:               bool,
    pub NoDamage:              bool,
    pub InfinitePoise:         bool,
    pub InfiniteStamina:       bool,
    pub InfiniteDurability:    bool,
    pub InfiniteConsumables:   bool,
    pub NoHollowing:           bool,
    pub NoSoulLoss:            bool,
    pub Hidden:                bool,
    pub Silent:                bool,
    pub SkipCredits:           bool,
    pub FastQuitout:           bool,
    pub DisableRoll:           bool,
    pub DisableBackstep:       bool,
    pub SkipIvoryKingGauntlet: bool,
    pub DisableLoyceKnights:   bool,
    pub StartEventLogger:      bool,
}
