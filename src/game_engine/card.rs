use anyhow::Error;
use rand::prelude::*;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub id: Id,
    pub attack: u8,
    pub defense: u8,
}

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct DeadCard {}
pub type OkOrDead<T> = Result<T, DeadCard>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Id(Uuid);
impl TryFrom<&str> for Id {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Error> {
        let s = Uuid::parse_str(s)?;
        Ok(Self(s))
    }
}

#[must_use]
pub fn generate() -> Card {
    let id = Id(Uuid::now_v7());
    let attack = random::<u8>() % 5;
    let defense = random::<u8>() % 5;
    Card {
        id,
        attack,
        defense,
    }
}

impl Card {
    #[allow(clippy::missing_errors_doc)]
    pub fn damage(&mut self, amount: u8) -> OkOrDead<()> {
        if amount >= self.defense {
            Err(DeadCard {})
        } else {
            self.defense -= amount;
            Ok(())
        }
    }
}
