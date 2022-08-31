#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
	if self.health > 0 {
	    return None;
	}
	if self.level >= 10 {
	    Some(Self {health:100, mana:Some(100), level:self.level})
	} else {
	    Some(Self {health:100, mana:None, level:self.level})
	}
	
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
	if let Some(mut mana) = self.mana.as_mut() {
	    if *mana < mana_cost {
		0
	    } else {
		*mana -= mana_cost;
		mana_cost * 2
	    }
	} else {
	    self.health = self.health.saturating_sub(mana_cost);
	    0
	}
    }
}
