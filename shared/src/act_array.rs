use {
    serde::{Deserialize, Serialize},
    std::{fmt::Display, str::FromStr},
    thiserror::Error,
};

const HIGHEST_ACT: u8 = 50;
const MAX_ACTS_AMOUNT: usize = 10;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ActArray {
    acts: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum ParseActArrayError {
    #[error("Expects integers seperated by spaces")]
    WrongFormat,
    #[error("Maximum number of acts is {MAX_ACTS_AMOUNT}")]
    ExceedsLimit,
    #[error("Highest act number is {HIGHEST_ACT}")]
    ExceedsHighestAct,
}

impl ActArray {
    pub fn as_dword_le_bytes(&self) -> Vec<u8> {
        let mut acts = self.acts.clone();
        acts.resize(MAX_ACTS_AMOUNT, 0);

        self.acts
            .iter()
            .flat_map(|&x| (x as i32).to_le_bytes())
            .collect()
    }
}

impl FromStr for ActArray {
    type Err = ParseActArrayError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let acts: Vec<u8> = input
            .split_whitespace()
            .map(|s| {
                let val = match s.parse::<u8>() {
                    Ok(val) => val,
                    Err(_) => return Err(ParseActArrayError::WrongFormat),
                };
                if val > HIGHEST_ACT {
                    return Err(ParseActArrayError::ExceedsHighestAct);
                }
                Ok(val)
            })
            .collect::<Result<Vec<_>, _>>()?;
        if acts.len() > MAX_ACTS_AMOUNT {
            return Err(ParseActArrayError::ExceedsLimit);
        }
        Ok(Self {
            acts,
        })
    }
}

impl Display for ActArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.acts)
    }
}
