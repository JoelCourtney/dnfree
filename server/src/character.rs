use serde::{Deserialize, Serialize};

use crate::feature::{Feature, Choose};
use crate::misc::*;
use std::fmt::Debug;
use macros::dynamic_choose;
use crate::content::Content;

#[derive(Debug, Deserialize, Serialize)]
pub struct StoredCharacter {
    name: String,

    health: u64,
    temp_health: u64,

    base_strength: u8,
    base_dexterity: u8,
    base_constitution: u8,
    base_intelligence: u8,
    base_wisdom: u8,
    base_charisma: u8,

    alignment: Alignment,

    race: Box<dyn Race>
}

impl StoredCharacter {
    pub fn resolve(&mut self) -> Character {
        let mut char = Character {
            name: self.name.clone(),
            health: self.health,
            temp_health: self.temp_health,

            strength: self.base_strength,
            dexterity: self.base_dexterity,
            constitution: self.base_constitution,
            intelligence: self.base_intelligence,
            wisdom: self.base_wisdom,
            charisma: self.base_charisma,

            alignment: self.alignment,

            ..Default::default()
        };
        self.race.initialize(&mut char);
        self.race.modify(&mut char);
        self.race.finalize(&mut char);
        char.race_traits.extend(self.race.features());
        char
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Character {
    pub name: String,

    // HEALTH
    pub health: u64,
    pub temp_health: u64,
    pub max_health: u64,

    // ABILITIES
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,

    pub strength_modifier: i8,
    pub dexterity_modifier: i8,
    pub constitution_modifier: i8,
    pub intelligence_modifier: i8,
    pub wisdom_modifier: i8,
    pub charisma_modifier: i8,

    // INITIATIVE
    pub initiative: i8,

    // SIZE
    pub size: CreatureSize,

    // ALIGNMENT
    pub alignment: Alignment,

    // PROFICIENCIES AND LANGUAGES
    pub skill_proficiencies: Vec<(Skill, ProficiencyType)>,
    pub languages: Vec<Language>,

    // SPEED
    pub walking_speed: u8,
    pub flying_speed: u8,
    pub climbing_speed: u8,
    pub swimming_speed: u8,
    pub burrowing_speed: u8,

    // FEATURES, TRAITS, AND FEATS
    pub race_traits: Vec<Feature>,
    pub class_features: Vec<Feature>,
    pub background_features: Vec<Feature>,
    pub feats: Vec<Vec<Feature>>,
}

#[dynamic_choose]
pub trait Race: Content + Debug {
    fn content_name(&self) -> &'static str;
}

#[dynamic_choose]
pub trait Class: Content + Debug {
    fn content_name(&self) -> &'static str;
}

#[dynamic_choose]
pub trait Feat: Content + Debug {
    fn content_name(&self) -> &'static str;
}