mod system;
mod official;
mod playtest;
mod homebrew;
mod custom_traits;
pub(crate) mod common;

use custom_traits::*;
use crate::feature::Feature;
use crate::character::Character;

macros::registry!(14);

/// Contains where to find a particular file.
///
/// collection = one of "official", "playtest", "homebrew"
/// source = name of book or homebrew creator
#[derive(Eq, PartialEq, Debug, Hash)]
struct Registration {
    collection: &'static str,
    source: &'static str,
}

pub trait Content {
    fn receive_choice(&mut self, _choice: &str, _feature_index: usize, _choice_index: usize) {
        unimplemented!()
    }
    fn features(&self) -> Vec<Feature> { vec! [] }

    fn receive_feat_choice(&mut self, _choice: &str, _feat_index: usize, _feature_index: usize, _choice_index: usize) -> Result<(),()> {
        Err(())
    }
    fn feats(&self) -> Vec<Vec<Feature>> { vec! [] }

    fn declare(&self, c: &mut Character);
    fn modify(&self, c: &mut Character);
}

pub trait LeveledContent {
    fn receive_choice(&mut self, _choice: &str, _feature_index: usize, _choice_index: usize) {
    unimplemented!()
    }
    fn features(&self, _level: usize) -> Vec<Feature> { vec! [] }

    fn receive_feat_choice(&mut self, _choice: &str, _feat_index: usize, _feature_index: usize, _choice_index: usize) -> Result<(),()> {
        Err(())
    }
    fn feats(&self, _level: usize) -> Vec<Vec<Feature>> { vec! [] }

    fn declare(&self, c: &mut Character, level: usize);
    fn modify(&self, c: &mut Character, level: usize);
}