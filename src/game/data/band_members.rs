use crate::common::prelude::*;
use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BandMember {
    Guitar,
    Drums,
    Flute,
    Accordion,
    Harmonica,
}

impl BandMember {
    pub fn len() -> usize {
        5
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Guitar,
            1 => Self::Drums,
            2 => Self::Flute,
            3 => Self::Accordion,
            4 => Self::Harmonica,
            _ => unreachable!(),
        }
    }

    pub fn selection_active_image(&self, asset_library: &AssetLibrary) -> Handle<Image> {
        match *self {
            Self::Guitar => asset_library
                .sprite_band_selection_slot_guitar_active
                .clone(),
            Self::Drums => asset_library
                .sprite_band_selection_slot_drums_active
                .clone(),
            Self::Flute => asset_library
                .sprite_band_selection_slot_guitar_active
                .clone(),
            Self::Accordion => asset_library
                .sprite_band_selection_slot_guitar_active
                .clone(),
            Self::Harmonica => asset_library
                .sprite_band_selection_slot_guitar_active
                .clone(),
        }
    }

    pub fn selection_inactive_image(&self, asset_library: &AssetLibrary) -> Handle<Image> {
        match *self {
            Self::Guitar => asset_library
                .sprite_band_selection_slot_guitar_inactive
                .clone(),
            Self::Drums => asset_library
                .sprite_band_selection_slot_drums_inactive
                .clone(),
            Self::Flute => asset_library
                .sprite_band_selection_slot_guitar_inactive
                .clone(),
            Self::Accordion => asset_library
                .sprite_band_selection_slot_guitar_inactive
                .clone(),
            Self::Harmonica => asset_library
                .sprite_band_selection_slot_guitar_inactive
                .clone(),
        }
    }
}
