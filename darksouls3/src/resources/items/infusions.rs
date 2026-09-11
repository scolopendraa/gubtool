use strum::Display;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Display)]
pub enum Infusion {
    Normal    = 0,
    Heavy     = 100,
    Sharp     = 200,
    Refined   = 300,
    Simple    = 400,
    Crystal   = 500,
    Fire      = 600,
    Chaos     = 700,
    Lightning = 800,
    Deep      = 900,
    Dark      = 1000,
    Poison    = 1100,
    Blood     = 1200,
    Raw       = 1300,
    Blessed   = 1400,
    Hollow    = 1500,
}

pub static INFUSIONS: [Infusion; 16] = [
    Infusion::Normal,
    Infusion::Heavy,
    Infusion::Sharp,
    Infusion::Refined,
    Infusion::Simple,
    Infusion::Crystal,
    Infusion::Fire,
    Infusion::Chaos,
    Infusion::Lightning,
    Infusion::Deep,
    Infusion::Dark,
    Infusion::Poison,
    Infusion::Blood,
    Infusion::Raw,
    Infusion::Blessed,
    Infusion::Hollow,
];
